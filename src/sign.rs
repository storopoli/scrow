//! Signs Taproot Transactions using Nostr keys.

use bitcoin::{
    Script, ScriptBuf, TapLeafHash, TapSighashType, Transaction, TxOut, Witness,
    hashes::Hash,
    key::TapTweak,
    sighash::{Prevouts, SighashCache},
    taproot::{LeafVersion, TaprootSpendInfo},
};
use dioxus::logger::tracing::{error, trace};
use nostr::key::{PublicKey as NostrPublicKey, SecretKey as NostrSecretKey};
use secp256k1::{Message, SECP256K1, schnorr};

use crate::{
    error::Error,
    scripts::{EscrowScript, escrow_scripts},
};

/// Signs a [`Transaction`] with the given [`NostrSecretKey`].
///
/// It must be a P2TR key path spend transaction with a single input as the 0th vout.
pub fn sign_resolution_tx(
    transaction: &Transaction,
    nsec: &NostrSecretKey,
    prevout: TxOut,
) -> Transaction {
    // Parse nsec to a bitcoin secret key.
    let keypair = nsec.keypair(SECP256K1);

    let mut sighasher = SighashCache::new(transaction);
    let sighash_type = TapSighashType::Default;
    let sighash = sighasher
        .taproot_key_spend_signature_hash(0, &Prevouts::All(&[prevout]), sighash_type)
        .expect("must create sighash");
    let message = Message::from_digest(*sighash.as_byte_array());

    // For key path spend, we need to apply taproot tweak.
    let tweaked = keypair.tap_tweak(SECP256K1, None);
    let signature = SECP256K1.sign_schnorr_no_aux_rand(&message, &tweaked.to_inner());
    trace!(signature = %signature, txid = %transaction.compute_txid(), "Signature resolution transaction");
    let mut transaction = transaction.clone();

    // Construct the witness stack
    let mut witness = Witness::new();
    witness.push(signature.as_ref());

    transaction.input[0].witness = witness;
    transaction
}

/// Signs an escrow P2TR [`Transaction`], given an input `index` using a [`NostrSecretKey`].
///
/// The input is signed using the provided [`NostrSecretKey`], `prevouts`, and [`ScriptBuf`] locking script.
#[expect(clippy::too_many_arguments)]
pub fn sign_escrow_tx(
    tx: &Transaction,
    index: usize,
    nsec: &NostrSecretKey,
    npub_1: &NostrPublicKey,
    npub_2: &NostrPublicKey,
    npub_arbitrator: Option<&NostrPublicKey>,
    timelock_duration: Option<u32>,
    prevouts: Vec<TxOut>,
    escrow_script: EscrowScript,
) -> Result<schnorr::Signature, Error> {
    // Parse nsec to a bitcoin secret key.
    let keypair = nsec.keypair(SECP256K1);

    // get which escrow type.
    let locking_script = escrow_scripts(
        npub_1,
        npub_2,
        npub_arbitrator,
        timelock_duration,
        escrow_script,
    )?;
    trace!(%index, locking_script = %locking_script.to_asm_string(), "escrow locking script");
    let leaf_hash = TapLeafHash::from_script(&locking_script, LeafVersion::TapScript);

    let sighash_type = TapSighashType::Default;
    let mut sighash_cache = SighashCache::new(tx);
    let sighash = sighash_cache
        .taproot_script_spend_signature_hash(
            index,
            &Prevouts::All(&prevouts),
            leaf_hash,
            sighash_type,
        )
        .expect("fail to create signhash");
    let message = Message::from_digest_slice(sighash.as_byte_array())?;

    // For script path, we use the UNTWEAKED keypair.
    let signature = SECP256K1.sign_schnorr_no_aux_rand(&message, &keypair);
    trace!(%index, %signature, txid = %tx.compute_txid(), "Signature escrow transaction");

    #[cfg(debug_assertions)]
    {
        let verification =
            SECP256K1.verify_schnorr(&signature, &message, &keypair.x_only_public_key().0);
        if verification.is_err() {
            error!("Signature verification failed: {:?}", verification.err());
        }
        assert!(verification.is_ok());
    }

    Ok(signature)
}

/// Types of escrow transactions.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EscrowType<'a> {
    /// Collaborative escrow transaction.
    ///
    /// No timelocks and no arbitrator.
    Collaborative {
        participant_1: &'a NostrPublicKey,
        participant_2: &'a NostrPublicKey,
    },

    /// Dispute escrow transaction.
    ///
    /// Timelocked and with an arbitrator.
    Dispute {
        participant_1: &'a NostrPublicKey,
        participant_2: &'a NostrPublicKey,
        arbitrator: &'a NostrPublicKey,
    },
}

/// Combine one multiple [`taproot::Signature`]s into a single [`Transaction`] input.
pub fn combine_signatures(
    mut transaction: Transaction,
    index: usize,
    signatures: Vec<&schnorr::Signature>,
    locking_script: &Script,
    taproot_spend_info: &TaprootSpendInfo,
) -> Transaction {
    let prevout_leaf = (ScriptBuf::from(locking_script), LeafVersion::TapScript);
    let control_block = taproot_spend_info
        .control_block(&prevout_leaf)
        .expect("Unable to create Control block");

    // Construct the witness stack
    let mut witness = Witness::new();

    // Push signatures in order
    for signature in signatures {
        witness.push(signature.as_ref());
    }

    // Push locking script
    witness.push(prevout_leaf.0.as_bytes());

    // Push control block
    witness.push(control_block.serialize());

    transaction.input[index].witness = witness;

    transaction
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use bitcoin::{
        Amount, BlockHash, Network, OutPoint, TxIn, absolute, consensus, hex::DisplayHex,
        transaction,
    };

    use corepc_node::Node;
    use dioxus::logger::tracing::debug;
    use nostr::nips::nip21::NostrURI;
    use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

    use crate::{
        scripts::{escrow_address, escrow_spend_info},
        tx::escrow_tx,
        util::{npub_to_address, npub_to_x_only_public_key},
    };

    use super::*;

    static COINBASE_AMOUNT: LazyLock<Amount> = LazyLock::new(|| Amount::from_btc(50.0).unwrap());
    const FEE: Amount = Amount::from_sat(1_000);
    static MULTISIG_AMOUNT: LazyLock<Amount> = LazyLock::new(|| *COINBASE_AMOUNT - FEE);
    // static ESCROW_AMOUNT: LazyLock<Amount> = LazyLock::new(|| *MULTISIG_AMOUNT - FEE);
    const COINBASE_MATURITY: usize = 101;

    // Generated by https://nostrtool.com
    // const NSEC_1: &str = "nsec1hufm8kzq0c4l9zsja7daynm47mfq2fkn38cm38yrpjmv6zctz2ysjmqw36";
    // const NPUB_1: &str = "npub1nckhhhcxm8usszvxt6yku6efp4fpay3saglx6yhtu8pfv3kdqhqsfn0vd7";
    // const NSEC_2: &str = "nsec1svda3gyta75ny0t7aqqv9ldh0hazt89qc48jjgw8wkv5wy9w6fgq34wv4z";
    // const NPUB_2: &str = "npub1xy4xk87gglf4psv3lr7aymvs09e44fq0zxcf6kc43lawusvz3cts270an7";

    fn generate_nostr_keys() -> (NostrSecretKey, NostrPublicKey) {
        let nsec = NostrSecretKey::generate();
        let npub: NostrPublicKey = nsec.public_key(SECP256K1).x_only_public_key().0.into();
        trace!(derived_npub = %npub.to_nostr_uri().unwrap());
        (nsec, npub)
    }

    #[test]
    fn sign_collaborative_tx_flow() {
        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(EnvFilter::from_default_env())
            .init();

        // Setup regtest node and clients.
        let bitcoind = Node::from_downloaded().unwrap();
        let btc_client = &bitcoind.client;

        // Get network.
        let network = btc_client
            .get_blockchain_info()
            .expect("must get blockchain info")
            .chain;
        let network = network.parse::<Network>().expect("network must be valid");

        // Generate nsec and npub.
        let (nsec_1, npub_1) = generate_nostr_keys();
        let (nsec_2, npub_2) = generate_nostr_keys();
        // Get the xonly pks.
        let xonly_1 = nsec_1.x_only_public_key(SECP256K1).0;
        let xonly_2 = nsec_2.x_only_public_key(SECP256K1).0;
        trace!(%xonly_1, %xonly_2, "xonly pks");

        // Fund a SegWit-v1 P2TR address from the npub.
        // Mine until maturity (101 blocks in Regtest).
        let funded_address = npub_to_address(&npub_1, network).unwrap();
        trace!(%funded_address, "Funded address");
        let coinbase_block = btc_client
            .generate_to_address(COINBASE_MATURITY, &funded_address)
            .expect("must be able to generate blocks")
            .0
            .first()
            .expect("must be able to get the blocks")
            .parse::<BlockHash>()
            .expect("must parse");
        let coinbase_txid = btc_client
            .get_block(coinbase_block)
            .expect("must be able to get coinbase block")
            .coinbase()
            .expect("must be able to get the coinbase transaction")
            .compute_txid();

        // Send to the 2-of-2 multisig address.
        let escrow_address = escrow_address(&npub_1, &npub_2, None, None, network).unwrap();
        trace!(%escrow_address, "Escrow address");

        // Create the transaction.
        let funding_input = OutPoint {
            txid: coinbase_txid,
            vout: 0,
        };
        let inputs = vec![TxIn {
            previous_output: funding_input,
            ..Default::default()
        }];
        let outputs = vec![TxOut {
            value: *MULTISIG_AMOUNT,
            script_pubkey: escrow_address.script_pubkey(),
        }];
        let unsigned = Transaction {
            version: transaction::Version(2),
            input: inputs,
            output: outputs,
            lock_time: absolute::LockTime::ZERO,
        };
        trace!(transaction=%consensus::serialize(&unsigned).as_hex(), "Unsigned funding transaction");

        // Sign the first input using Sighashes
        let prevouts = TxOut {
            value: *COINBASE_AMOUNT,
            script_pubkey: funded_address.script_pubkey(),
        };
        let signed = sign_resolution_tx(&unsigned, &nsec_1, prevouts);
        trace!(transaction=%consensus::serialize(&signed).as_hex(), "Signed funding");

        // Test if the transaction is valid.
        let result = btc_client.send_raw_transaction(&signed);
        assert!(result.is_ok());
        let txid = result.unwrap().txid().unwrap();
        assert_eq!(txid, signed.compute_txid());
        debug!(%txid, "Sent to the escrow address");
        // Mine 1 block to mine the transaction
        btc_client.generate_to_address(1, &funded_address).unwrap();

        // Spend from the escrow address.
        let unsigned = escrow_tx(
            &npub_1,
            &npub_2,
            None,
            *MULTISIG_AMOUNT / 2,
            *MULTISIG_AMOUNT / 2,
            txid,
            FEE,
            network,
        )
        .unwrap();
        trace!(transaction=%consensus::serialize(&unsigned).as_hex(), "Unsigned escrow transaction");

        let script_pubkey = escrow_address.script_pubkey();
        let prevouts = TxOut {
            value: *MULTISIG_AMOUNT, // Changed from ESCROW_AMOUNT to MULTISIG_AMOUNT
            script_pubkey,
        };
        let sig_1 = sign_escrow_tx(
            &unsigned,
            0,
            &nsec_1,
            &npub_1,
            &npub_2,
            None,
            None,
            vec![prevouts.clone()],
            EscrowScript::A,
        )
        .unwrap();
        let sig_2 = sign_escrow_tx(
            &unsigned,
            0,
            &nsec_2,
            &npub_1,
            &npub_2,
            None,
            None,
            vec![prevouts.clone()],
            EscrowScript::A,
        )
        .unwrap();

        // Manually verify each signature
        let locking_script = escrow_scripts(&npub_1, &npub_2, None, None, EscrowScript::A).unwrap();
        trace!(locking_script=%locking_script.to_asm_string(), "Locking script");
        let tap_leaf_hash = TapLeafHash::from_script(&locking_script, LeafVersion::TapScript);
        let sighash = SighashCache::new(&unsigned)
            .taproot_script_spend_signature_hash(
                0,
                &Prevouts::All(&[prevouts.clone()]),
                tap_leaf_hash,
                TapSighashType::Default,
            )
            .expect("Failed to create sighash");
        let message = Message::from_digest_slice(sighash.as_byte_array()).unwrap();

        // Verify each signature individually
        let xonly_pk1 = npub_to_x_only_public_key(&npub_1).unwrap();
        let xonly_pk2 = npub_to_x_only_public_key(&npub_2).unwrap();
        let verify1 = SECP256K1.verify_schnorr(&sig_1, &message, &xonly_pk1);
        let verify2 = SECP256K1.verify_schnorr(&sig_2, &message, &xonly_pk2);
        assert!(verify1.is_ok() && verify2.is_ok());

        let script_ver = &(locking_script.clone(), LeafVersion::TapScript);
        trace!(locking_script=%script_ver.0.to_asm_string(), leaf_version=%script_ver.1, "Script version");
        let taproot_spend_info = escrow_spend_info(&npub_1, &npub_2, None, None).unwrap();
        let signed = combine_signatures(
            unsigned,
            0,
            vec![&sig_1, &sig_2],
            &locking_script,
            &taproot_spend_info,
        );
        trace!(transaction=%consensus::serialize(&signed).as_hex(), "Signed escrow");
        let result = btc_client.send_raw_transaction(&signed);
        assert!(result.is_ok());
    }
}
