//! Signs Taproot Transactions using Nostr keys.

use bitcoin::{
    Script, TapLeafHash, TapSighashType, Transaction, TxOut,
    hashes::Hash,
    sighash::{Prevouts, SighashCache},
    taproot::{ControlBlock, LeafVersion},
};
use dioxus::logger::tracing::trace;
use nostr::key::{PublicKey as NostrPublicKey, SecretKey as NostrSecretKey};
use secp256k1::{Keypair, Message, SECP256K1, SecretKey, schnorr};

use crate::{
    error::Error,
    scripts::{EscrowScript, escrow_scripts},
};

/// Signs an escrow P2TR [`Transaction`], given an input `index` using a [`NostrSecretKey`].
///
/// The input is signed using the provided [`NostrSecretKey`], `prevouts`, and [`ScriptBuf`] locking script.
#[allow(clippy::too_many_arguments)]
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
    let sk = SecretKey::from_slice(&nsec.to_secret_bytes()).expect("infallible");
    let keypair = Keypair::from_secret_key(SECP256K1, &sk);

    // get which escrow type.
    let locking_script = escrow_scripts(
        npub_1,
        npub_2,
        npub_arbitrator,
        timelock_duration,
        escrow_script,
    )?;
    let leaf_hash = TapLeafHash::from_script(locking_script.as_script(), LeafVersion::TapScript);

    let sighash_type = TapSighashType::All;
    let mut sighash_cache = SighashCache::new(tx);
    let sighash = sighash_cache
        .taproot_script_spend_signature_hash(
            index,
            &Prevouts::All(&prevouts),
            leaf_hash,
            sighash_type,
        )
        .unwrap();
    let message =
        Message::from_digest_slice(sighash.as_byte_array()).expect("must create a message");
    let signature = SECP256K1.sign_schnorr(&message, &keypair);
    trace!(%index, %signature, "Signature ");

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

/// Combine one multiple [`schnorr::Signature`]s into a single [`Transaction`] input.
pub fn combine_signatures(
    tx: Transaction,
    index: usize,
    signatures: Vec<&schnorr::Signature>,
    locking_script: &Script,
    control_block: ControlBlock,
) -> Transaction {
    let mut transaction = tx;

    // Push signatures in order
    for signature in signatures {
        transaction.input[index].witness.push(signature.as_ref());
    }

    // Push locking script
    transaction.input[index]
        .witness
        .push(locking_script.to_bytes());

    // Push control block
    transaction.input[index]
        .witness
        .push(control_block.serialize());

    transaction
}

#[cfg(test)]
mod tests {
    use bitcoin::Amount;

    use secp256k1::{SecretKey, rand::thread_rng};

    use super::*;

    const COINBASE_AMOUNT: Amount = Amount::from_sat(50_000_000);
    const FEE: Amount = Amount::from_sat(1_000);
    const MULTISIG_AMOUNT: Amount = Amount::from_sat(49_999_000);
    // Split escrow into 2 equal parts for tests.
    const ESCROW_AMOUNT: Amount = Amount::from_sat(49_998_000);
    const COINBASE_MATURITY: usize = 101;

    fn generate_nostr_keys() -> (NostrSecretKey, NostrPublicKey) {
        let mut rng = thread_rng();
        let sk = SecretKey::new(&mut rng);
        let nsec = NostrSecretKey::from_slice(&sk.secret_bytes()).unwrap();
        let (x_only_pk, _) = nsec.public_key(SECP256K1).x_only_public_key();
        let npub = NostrPublicKey::from_slice(&x_only_pk.serialize()).unwrap();
        (nsec, npub)
    }

    /*
    #[test]
    fn sign_p2wsh_collaborative_tx_flow() {
        env_logger::init();

        // Setup regtest node and clients.
        let bitcoind = Node::from_downloaded().unwrap();
        let btc_client = &bitcoind.client;

        // Get network.
        let network = btc_client
            .get_blockchain_info()
            .expect("must get blockchain info")
            .chain;
        let network = network.parse::<Network>().expect("network must be valid");

        // Generate the sk and pk
        let (nsec_1, npub_1) = generate_nostr_keys();
        let (nsec_2, npub_2) = generate_nostr_keys();

        // Fund a SegWit-v0 address from the PublicKey.
        // Mine until maturity (101 blocks in Regtest).
        let funded_address = npub_to_address(&npub_1, network).unwrap();
        println!("Funded address: {}", funded_address);
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
        // We're sending 49.999 and 0.001 will be fees.
        let multisig_address = collaborative_address(&[&npub_1, &npub_2], network);
        println!("Multisig address: {}", multisig_address);

        // Commutative check.
        assert_eq!(
            collaborative_address(&[&npub_1, &npub_2], network),
            collaborative_address(&[&npub_2, &npub_1], network),
        );

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
            value: MULTISIG_AMOUNT,
            script_pubkey: multisig_address.script_pubkey(),
        }];
        let unsigned = Transaction {
            version: transaction::Version(2),
            input: inputs,
            output: outputs,
            lock_time: LockTime::ZERO,
        };
        println!(
            "Unsigned funding transaction: {}",
            consensus::serialize(&unsigned).as_hex()
        );

        // Sign the first input using Sighashes
        let spk = funded_address.script_pubkey();
        let sighash_type = EcdsaSighashType::All;
        let mut sighash_cache = SighashCache::new(unsigned);
        let sighash = sighash_cache
            .p2wpkh_signature_hash(0, &spk, COINBASE_AMOUNT, sighash_type)
            .unwrap();
        let message = Message::from(sighash);
        let btc_sk_1 = SecretKey::from_slice(&nsec_1.secret_bytes()).unwrap();
        let btc_pk_1 = npub_to_x_only_public_key(&npub_1).unwrap();
        let signature = SECP256K1.sign_ecdsa(&message, &btc_sk_1);
        // Update the witness stack
        let signature = ecdsa::Signature {
            signature,
            sighash_type,
        };
        *sighash_cache.witness_mut(0).unwrap() = Witness::p2wpkh(&signature, &btc_pk_1.inner);
        let signed_tx = sighash_cache.into_transaction();
        println!("Signed funding transaction: {:?}", signed_tx);

        // Test if the transaction is valid.
        let result = btc_client.send_raw_transaction(&signed_tx);
        assert!(result.is_ok());
        let txid = result.unwrap().txid().unwrap();
        assert_eq!(txid, signed_tx.compute_txid());
        println!("Transaction ID: {}", txid);
        // Mine 1 block to mine the transaction
        btc_client.generate_to_address(1, &funded_address).unwrap();

        // Spend from the 2-of-2 collaborative address.
        let final_address = btc_client.new_address().unwrap();
        let unsigned_tx = Transaction {
            version: transaction::Version(2),
            input: vec![TxIn {
                previous_output: OutPoint { txid, vout: 0 },
                ..Default::default()
            }],
            output: vec![TxOut {
                value: ESCROW_AMOUNT,
                script_pubkey: final_address.script_pubkey(),
            }],
            lock_time: LockTime::ZERO,
        };
        let script_pubkey = multisig_address.script_pubkey();
        assert!(script_pubkey.is_p2wsh());
        println!("ScriptPubKey: {}", script_pubkey);

        let unlocking_script = collaborative_unlocking_script(&[&npub_1, &npub_2]);
        println!("Unlocking Script: {}", unlocking_script);

        // Commutative check.
        assert_eq!(
            collaborative_unlocking_script(&[&npub_1, &npub_2]),
            collaborative_unlocking_script(&[&npub_2, &npub_1]),
        );
        let sig_1 = sign_tx(
            unsigned_tx.clone(),
            0,
            &nsec_1,
            MULTISIG_AMOUNT,
            &unlocking_script,
        );
        let sig_2 = sign_tx(
            unsigned_tx.clone(),
            0,
            &nsec_2,
            MULTISIG_AMOUNT,
            &unlocking_script,
        );
        let signed_tx = combine_signatures_collaborative(
            unsigned_tx,
            0,
            vec![sig_1, sig_2],
            vec![npub_1, npub_2],
            unlocking_script,
        );
        assert!(signed_tx.input[0].witness.witness_script().is_some());
        println!(
            "Signed transaction: {}",
            consensus::serialize(&signed_tx).as_hex()
        );
        let result = btc_client.send_raw_transaction(&signed_tx);
        assert!(result.is_ok());
    }
    */

    /*
    #[test]
    fn sign_p2wsh_dispute_no_arbitrator_tx_flow() {
        // Setup regtest node and clients.
        let bitcoind = Node::from_downloaded().unwrap();
        let btc_client = &bitcoind.client;

        // Get network.
        let network = btc_client
            .get_blockchain_info()
            .expect("must get blockchain info")
            .chain;
        let network = network.parse::<Network>().expect("network must be valid");

        // Generate the sk and pk
        let (nsec_1, npub_1) = generate_nostr_keys();
        let (nsec_2, npub_2) = generate_nostr_keys();
        let (_nsec_arb, npub_arb) = generate_nostr_keys();

        // Fund a SegWit-v0 address from the PublicKey.
        // Mine until maturity (101 blocks in Regtest).
        let funded_address = npub_to_address(&npub_1, network).unwrap();
        println!("Funded address: {}", funded_address);
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

        // Send to the 2-of-3 multisig address.
        // We're sending 49.999 and 0.001 will be fees.
        let timelock_duration = 10;
        let multisig_address =
            dispute_address(&[&npub_1, &npub_2], &npub_arb, timelock_duration, network);
        println!("Multisig address: {}", multisig_address);

        // Commutative check.
        assert_eq!(
            dispute_address(&[&npub_1, &npub_2], &npub_arb, timelock_duration, network,),
            dispute_address(&[&npub_1, &npub_2], &npub_arb, timelock_duration, network,),
        );

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
            value: MULTISIG_AMOUNT,
            script_pubkey: multisig_address.script_pubkey(),
        }];
        let unsigned = Transaction {
            version: transaction::Version(2),
            input: inputs,
            output: outputs,
            lock_time: LockTime::ZERO,
        };
        println!(
            "Unsigned funding transaction: {}",
            consensus::serialize(&unsigned).as_hex()
        );

        // Sign the first input using Sighashes
        let spk = funded_address.script_pubkey();
        let sighash_type = EcdsaSighashType::All;
        let mut sighash_cache = SighashCache::new(unsigned);
        let sighash = sighash_cache
            .p2wpkh_signature_hash(0, &spk, COINBASE_AMOUNT, sighash_type)
            .unwrap();
        let message = Message::from(sighash);
        let btc_sk_1 = SecretKey::from_slice(&nsec_1.secret_bytes()).unwrap();
        let btc_pk_1 = npub_to_x_only_public_key(&npub_1).unwrap();
        let signature = SECP256K1.sign_ecdsa(&message, &btc_sk_1);
        // Update the witness stack
        let signature = ecdsa::Signature {
            signature,
            sighash_type,
        };
        *sighash_cache.witness_mut(0).unwrap() = Witness::p2wpkh(&signature, &btc_pk_1.inner);
        let signed_tx = sighash_cache.into_transaction();
        println!("Signed funding transaction: {:?}", signed_tx);

        // Test if the transaction is valid.
        let result = btc_client.send_raw_transaction(&signed_tx);
        assert!(result.is_ok());
        let txid = result.unwrap().txid().unwrap();
        assert_eq!(txid, signed_tx.compute_txid());
        println!("Transaction ID: {}", txid);
        // Mine 1 block to mine the transaction
        btc_client
            .generate_to_address(timelock_duration as usize, &funded_address)
            .unwrap();

        // Spend from the 2-of-3 dispute address.
        let final_address = btc_client.new_address().unwrap();
        let unsigned_tx = Transaction {
            version: transaction::Version(2),
            input: vec![TxIn {
                previous_output: OutPoint { txid, vout: 0 },
                ..Default::default()
            }],
            output: vec![TxOut {
                value: ESCROW_AMOUNT,
                script_pubkey: final_address.script_pubkey(),
            }],
            lock_time: LockTime::ZERO,
        };
        let script_pubkey = multisig_address.script_pubkey();
        assert!(script_pubkey.is_p2wsh());
        println!("ScriptPubKey: {}", script_pubkey);

        let unlocking_script =
            dispute_unlocking_script(&[&npub_1, &npub_2], &npub_arb, timelock_duration);
        println!("Unlocking Script: {}", unlocking_script);

        // Commutative check.
        assert_eq!(
            dispute_unlocking_script(&[&npub_1, &npub_2], &npub_arb, timelock_duration,),
            dispute_unlocking_script(&[&npub_2, &npub_1], &npub_arb, timelock_duration,),
        );

        let sig_1 = sign_tx(
            unsigned_tx.clone(),
            0,
            &nsec_1,
            MULTISIG_AMOUNT,
            &unlocking_script,
        );
        let sig_2 = sign_tx(
            unsigned_tx.clone(),
            0,
            &nsec_2,
            MULTISIG_AMOUNT,
            &unlocking_script,
        );
        let signed_tx = combine_signatures_dispute_collaborative(
            unsigned_tx,
            0,
            vec![sig_1, sig_2],
            vec![npub_1, npub_2],
            unlocking_script,
        );
        assert!(signed_tx.input[0].witness.witness_script().is_some());
        println!(
            "Signed transaction: {}",
            consensus::serialize(&signed_tx).as_hex()
        );
        let result = btc_client.send_raw_transaction(&signed_tx);
        assert!(result.is_ok());
    }
    */

    /*
    #[test]
    fn sign_p2wsh_dispute_with_arbitrator_tx_flow_1() {
        env_logger::init();

        // Setup regtest node and clients.
        let bitcoind = Node::from_downloaded().unwrap();
        let btc_client = &bitcoind.client;

        // Get network.
        let network = btc_client
            .get_blockchain_info()
            .expect("must get blockchain info")
            .chain;
        let network = network.parse::<Network>().expect("network must be valid");

        // Get the PrivateKey and PublicKeys from constants.
        let sec_key1 = PRIVATE_KEY1_HEX
            .parse::<SecretKey>()
            .expect("must parse secret key");
        let private_key1 = PrivateKey::new(sec_key1, network);
        let public_key1 = private_key1.public_key(SECP256K1);
        assert_eq!(
            private_key1.public_key(SECP256K1).to_string(),
            PUBLIC_KEY1_HEX
        );
        let sec_key2 = PRIVATE_KEY2_HEX
            .parse::<SecretKey>()
            .expect("must parse secret key");
        let private_key2 = PrivateKey::new(sec_key2, network);
        let public_key2 = private_key2.public_key(SECP256K1);
        assert_eq!(
            private_key2.public_key(SECP256K1).to_string(),
            PUBLIC_KEY2_HEX
        );
        let sec_key_third = PRIVATE_KEY3_HEX
            .parse::<SecretKey>()
            .expect("must parse secret key");
        let private_key_third = PrivateKey::new(sec_key_third, network);
        let public_key_third = private_key_third.public_key(SECP256K1);
        assert_eq!(
            private_key_third.public_key(SECP256K1).to_string(),
            PUBLIC_KEY3_HEX
        );

        // Fund a SegWit-v0 address from the PublicKey.
        // Mine until maturity (101 blocks in Regtest).
        let compressed_pk: CompressedPublicKey = public_key1.try_into().unwrap();
        let funded_address = Address::p2wpkh(&compressed_pk, network);
        println!("Funded address: {}", funded_address);
        let coinbase_block = btc_client
            .generate_to_address(101, &funded_address)
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

        // Send to the 2-of-3 multisig address.
        // We're sending 49.999 and 0.001 will be fees.
        let multisig_amount = Amount::from_btc(49.999).unwrap();
        let timelock_duration = 10;
        let multisig_address = new_dispute_address(
            [public_key1, public_key2],
            public_key_third,
            timelock_duration,
            network,
        );
        assert_eq!(
            new_dispute_address(
                [public_key1, public_key2],
                public_key_third,
                timelock_duration,
                network,
            ),
            new_dispute_address(
                [public_key2, public_key1],
                public_key_third,
                timelock_duration,
                network,
            ),
        );
        println!("Multisig address: {}", multisig_address);

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
            value: multisig_amount,
            script_pubkey: multisig_address.script_pubkey(),
        }];
        let unsigned = Transaction {
            version: transaction::Version(2),
            input: inputs,
            output: outputs,
            lock_time: LockTime::ZERO,
        };
        println!(
            "Unsigned funding transaction: {}",
            consensus::serialize(&unsigned).as_hex()
        );

        // Sign the first input using Sighashes
        let spk = funded_address.script_pubkey();
        let coinbase_amount = Amount::from_btc(50.0).unwrap();
        let sighash_type = EcdsaSighashType::All;
        let mut sighash_cache = SighashCache::new(unsigned);
        let sighash = sighash_cache
            .p2wpkh_signature_hash(0, &spk, coinbase_amount, sighash_type)
            .unwrap();
        let message = Message::from(sighash);
        let signature = SECP256K1.sign_ecdsa(&message, &private_key1.inner);
        // Update the witness stack
        let signature = ecdsa::Signature {
            signature,
            sighash_type,
        };
        *sighash_cache.witness_mut(0).unwrap() = Witness::p2wpkh(&signature, &public_key1.inner);
        let signed_tx = sighash_cache.into_transaction();
        println!("Signed funding transaction: {:?}", signed_tx);

        // Test if the transaction is valid.
        let result = btc_client.send_raw_transaction(&signed_tx);
        assert!(result.is_ok());
        let txid = result.unwrap().txid().unwrap();
        assert_eq!(txid, signed_tx.compute_txid());
        println!("Transaction ID: {}", txid);
        // Mine 1 block to mine the transaction
        btc_client
            .generate_to_address(timelock_duration as usize, &funded_address)
            .unwrap();

        // Spend from the 2-of-3 dispute address.
        let final_address = btc_client.new_address().unwrap();
        // Again 0.001 fees.
        let final_amount = Amount::from_btc(49.998).unwrap();
        let unsigned_tx = Transaction {
            version: transaction::Version(2),
            input: vec![TxIn {
                previous_output: OutPoint { txid, vout: 0 },
                sequence: Sequence::from_consensus(timelock_duration),
                ..Default::default()
            }],
            output: vec![TxOut {
                value: final_amount,
                script_pubkey: final_address.script_pubkey(),
            }],
            lock_time: LockTime::ZERO,
        };
        let script_pubkey = multisig_address.script_pubkey();
        assert!(script_pubkey.is_p2wsh());
        println!("ScriptPubKey: {}", script_pubkey);

        let unlocking_script = new_dispute_unlocking_script(
            [public_key1, public_key2],
            public_key_third,
            timelock_duration,
        );
        assert_eq!(
            new_dispute_unlocking_script(
                [public_key1, public_key2],
                public_key_third,
                timelock_duration,
            ),
            new_dispute_unlocking_script(
                [public_key2, public_key1],
                public_key_third,
                timelock_duration,
            ),
        );
        println!("Unlocking Script: {}", unlocking_script);
        let sig_1 = sign_tx(
            unsigned_tx.clone(),
            0,
            private_key1,
            multisig_amount,
            unlocking_script.clone(),
        );
        let sig_third = sign_tx(
            unsigned_tx.clone(),
            0,
            private_key_third,
            multisig_amount,
            unlocking_script.clone(),
        );
        let signed_tx = combine_signatures_dispute_arbitrator(
            unsigned_tx,
            0,
            vec![sig_1, sig_third],
            vec![public_key1, public_key_third],
            unlocking_script,
        );
        assert!(signed_tx.input[0].witness.witness_script().is_some());
        println!(
            "Signed transaction: {}",
            consensus::serialize(&signed_tx).as_hex()
        );
        let result = btc_client.send_raw_transaction(&signed_tx);
        assert!(result.is_ok());
    }
    */
}
