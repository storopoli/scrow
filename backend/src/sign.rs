use std::collections::BTreeMap;

use bitcoin::{
    ecdsa, sighash::SighashCache, Amount, EcdsaSighashType, PrivateKey, Psbt, PublicKey, ScriptBuf,
    Transaction, TxOut,
};
use secp256k1::{Message, SECP256K1};

/// Signs a [`Psbt`] with a given [`PrivateKey`] and XYZ.
///
/// This expects that all the inputs are SegWit-v0.
///
/// # Implementation Details
///
/// Derives the [`PublicKey`] from the [`PrivateKey`] and inserts
/// it into a [`BTreeMap`] so that the [`Psbt::sign`] method can use it.
#[deprecated = "Use `sign_tx` instead"]
pub fn sign_psbt(
    mut psbt: Psbt,
    private_key: PrivateKey,
    prevout: TxOut,
    redeem_script: ScriptBuf,
) -> Psbt {
    // Insert UTXO information into the Psbt
    psbt.inputs[0].witness_utxo = Some(prevout);
    psbt.inputs[0].sighash_type = Some(EcdsaSighashType::All.into());
    psbt.inputs[0].redeem_script = Some(redeem_script);

    // Get the BTreeMap<PublicKey, PrivateKey> for signing.
    let public_key = private_key.public_key(SECP256K1);
    let mut map: BTreeMap<PublicKey, PrivateKey> = BTreeMap::new();
    map.insert(public_key, private_key);
    psbt.sign(&map, SECP256K1).unwrap();

    psbt
}

/// Sign a [`Transaction`] input `index` using a [`PrivateKey`].
///
/// The input is signed using the provided [`PrivateKey`], [`Amount`], and [`ScriptBuf`] locking script.
pub fn sign_tx_collaborative(
    tx: Transaction,
    index: usize,
    private_key: PrivateKey,
    amount: Amount,
    unlocking_script: ScriptBuf,
) -> ecdsa::Signature {
    let sighash_type = EcdsaSighashType::All;
    let mut sighash_cache = SighashCache::new(tx);
    let sighash = sighash_cache
        .p2wsh_signature_hash(index, &unlocking_script, amount, sighash_type)
        .unwrap();
    let message = Message::from(sighash);
    let signature = SECP256K1.sign_ecdsa(&message, &private_key.inner);
    ecdsa::Signature {
        signature,
        sighash_type,
    }
}

/// Combine multiple [`ecdsa::Signature`]s into a single [`Transaction`] input.
///
/// It also sorts the PKs to see which one is the first to go into the witness stack,
/// given a sorted multisig script.
pub fn combine_signatures(
    tx: Transaction,
    index: usize,
    signatures: Vec<ecdsa::Signature>,
    pks: Vec<PublicKey>,
    unlocking_script: ScriptBuf,
    collaborative: bool,
) -> Transaction {
    let mut transaction = tx;
    // Collaborative means 2-of-2 multisig.
    // And we need a fucking empty thing first.
    if collaborative {
        transaction.input[index].witness.push([]);
    }

    // Create pairs of PKs and signatures and sort by PK
    let mut pairs: Vec<(PublicKey, ecdsa::Signature)> =
        pks.into_iter().zip(signatures.into_iter()).collect();

    // Sort pairs based on public keys
    pairs.sort_by(|a, b| a.0.cmp(&b.0));

    // Push signatures in order of sorted public keys
    for (_, signature) in pairs {
        transaction.input[index].witness.push(signature.serialize());
    }

    // Finally, push the unlocking script
    transaction.input[index].witness.push(&unlocking_script);
    transaction
}

#[cfg(test)]
mod tests {
    use bitcoin::{
        absolute::LockTime, consensus, ecdsa, hex::DisplayHex, sighash::SighashCache, transaction,
        Address, Amount, BlockHash, CompressedPublicKey, Network, OutPoint, Transaction, TxIn,
        TxOut, Witness,
    };
    use corepc_node::Node;
    use miniscript::ToPublicKey;
    use secp256k1::{Message, Parity, SecretKey};

    use crate::scripts::{new_collaborative_address, new_collaborative_unlocking_script};

    use super::*;

    // Taken from utils.rs test cases.
    const PRIVATE_KEY1_HEX: &str =
        "67dea2ed018072d675f5415ecfaed7d2597555e202d85b3d65ea4e58d2d92ffa";
    // NOTE: it is uncompressed prepended with 0x02 for the Even parity.
    const PUBLIC_KEY1_HEX: &str =
        "027e7e9c42a91bfef19fa929e5fda1b72e0ebc1a4c1141673e2794234d86addf4e";
    const PRIVATE_KEY2_HEX: &str =
        "30e8a8ea9f4402731d43ebc0aa34bb2812d5f255324e1fc6773a87f40af50aa4";
    // NOTE: it is uncompressed prepended with 0x02 for the Even parity.
    const PUBLIC_KEY2_HEX: &str =
        "021e0081633de90cc312d507416a6f1f056980cfb131b3ae64a0c953017f0f494f";

    // NOTE: print debug traces to introspect the Psbt.
    #[test]
    #[ignore = "PSBTs don't work, fuck'em"]
    fn sign_psbt_flow() {
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
        let sec_key = PRIVATE_KEY1_HEX
            .parse::<SecretKey>()
            .expect("must parse secret key");
        let private_key = PrivateKey::new(sec_key, network);
        let public_key = private_key.public_key(&SECP256K1);
        assert_eq!(
            private_key.public_key(SECP256K1).to_string(),
            PUBLIC_KEY1_HEX
        );

        // Fund a SegWit-v0 address from the PublicKey.
        // Mine until maturity (101 blocks in Regtest).
        let compressed_pk: CompressedPublicKey = public_key.try_into().unwrap();
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

        // We're sending 49.999 and 0.001 will be fees.
        let send_amount = Amount::from_btc(49.999).unwrap();
        let send_address = btc_client.new_address().unwrap();

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
            value: send_amount,
            script_pubkey: send_address.script_pubkey(),
        }];
        let unsigned = Transaction {
            version: transaction::Version(2),
            input: inputs,
            output: outputs,
            lock_time: LockTime::ZERO,
        };
        let psbt = Psbt::from_unsigned_tx(unsigned).expect("it is unsigned");
        println!("Unsigned PSBT: {psbt:?}");

        // Sign the Psbt.
        let public_key = private_key.public_key(&SECP256K1);
        let mut map: BTreeMap<PublicKey, PrivateKey> = BTreeMap::new();
        map.insert(public_key, private_key);
        // Get the Prevout.
        let coinbase_amount = Amount::from_btc(50.0).unwrap();
        let prevout = TxOut {
            value: coinbase_amount,
            script_pubkey: funded_address.script_pubkey(),
        };
        // Get the Redeem Script.
        let redeem_script = funded_address.script_pubkey();
        #[allow(deprecated)]
        let mut signed_psbt = sign_psbt(psbt, private_key, prevout, redeem_script);
        println!("Signed PSBT: {signed_psbt:?}");

        // let finalized_psbt = signed_psbt.finalize(SECP256K1).unwrap();
        let final_script_witness = {
            let (_, sig) = signed_psbt.inputs[0].partial_sigs.iter().next().unwrap();
            let pk = secp256k1::PublicKey::from_x_only_public_key(
                public_key.to_x_only_pubkey(),
                Parity::Even,
            );
            Witness::p2wpkh(sig, &pk)
        };
        // Clear all the data fields as per the spec.
        signed_psbt.inputs[0].final_script_witness = Some(final_script_witness);
        signed_psbt.inputs[0].partial_sigs = BTreeMap::new();
        signed_psbt.inputs[0].sighash_type = None;
        signed_psbt.inputs[0].redeem_script = None;
        signed_psbt.inputs[0].witness_script = None;
        signed_psbt.inputs[0].bip32_derivation = BTreeMap::new();
        println!("Finalized PSBT: {signed_psbt:?}");

        // Test if the transaction is valid.
        let signed_tx = signed_psbt.extract_tx().unwrap();
        let result = btc_client.send_raw_transaction(&signed_tx);
        assert!(result.is_ok());
    }

    #[test]
    fn sign_p2wpkh_tx_flow() {
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
        let sec_key = PRIVATE_KEY1_HEX
            .parse::<SecretKey>()
            .expect("must parse secret key");
        let private_key = PrivateKey::new(sec_key, network);
        let public_key = private_key.public_key(&SECP256K1);
        assert_eq!(
            private_key.public_key(SECP256K1).to_string(),
            PUBLIC_KEY1_HEX
        );

        // Fund a SegWit-v0 address from the PublicKey.
        // Mine until maturity (101 blocks in Regtest).
        let compressed_pk: CompressedPublicKey = public_key.try_into().unwrap();
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

        // We're sending 49.999 and 0.001 will be fees.
        let send_amount = Amount::from_btc(49.999).unwrap();
        let send_address = btc_client.new_address().unwrap();

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
            value: send_amount,
            script_pubkey: send_address.script_pubkey(),
        }];
        let unsigned = Transaction {
            version: transaction::Version(2),
            input: inputs,
            output: outputs,
            lock_time: LockTime::ZERO,
        };

        // Sign the first input using Sighashes
        let spk = funded_address.script_pubkey();
        let coinbase_amount = Amount::from_btc(50.0).unwrap();
        let sighash_type = EcdsaSighashType::All.into();
        let mut sighash_cache = SighashCache::new(unsigned);
        let sighash = sighash_cache
            .p2wpkh_signature_hash(0, &spk, coinbase_amount, sighash_type)
            .unwrap();
        let message = Message::from(sighash);
        let signature = SECP256K1.sign_ecdsa(&message, &private_key.inner);
        // Update the witness stack
        let signature = ecdsa::Signature {
            signature,
            sighash_type,
        };
        *sighash_cache.witness_mut(0).unwrap() = Witness::p2wpkh(&signature, &public_key.inner);
        let signed_tx = sighash_cache.into_transaction();
        println!("Signed transaction: {:?}", signed_tx);
        println!(
            "Signed transaction: {}",
            consensus::serialize(&signed_tx).as_hex()
        );

        // Test if the transaction is valid.
        let result = btc_client.send_raw_transaction(&signed_tx);
        assert!(result.is_ok());
    }

    #[test]
    fn sign_p2wsh_tx_flow() {
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
        let public_key1 = private_key1.public_key(&SECP256K1);
        assert_eq!(
            private_key1.public_key(SECP256K1).to_string(),
            PUBLIC_KEY1_HEX
        );
        let sec_key2 = PRIVATE_KEY2_HEX
            .parse::<SecretKey>()
            .expect("must parse secret key");
        let private_key2 = PrivateKey::new(sec_key2, network);
        let public_key2 = private_key2.public_key(&SECP256K1);
        assert_eq!(
            private_key2.public_key(SECP256K1).to_string(),
            PUBLIC_KEY2_HEX
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

        // Send to the 2-of-2 multisig address.
        // We're sending 49.999 and 0.001 will be fees.
        let multisig_amount = Amount::from_btc(49.999).unwrap();
        let multisig_address = new_collaborative_address([public_key1, public_key2], network);
        assert_eq!(
            new_collaborative_address([public_key1, public_key2], network),
            new_collaborative_address([public_key2, public_key1], network),
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
        let sighash_type = EcdsaSighashType::All.into();
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
        btc_client.generate_to_address(1, &funded_address).unwrap();

        // Spend from the 2-of-2 collaborative address.
        let final_address = btc_client.new_address().unwrap();
        // Again 0.001 fees.
        let final_amount = Amount::from_btc(49.998).unwrap();
        let unsigned_tx = Transaction {
            version: transaction::Version(2),
            input: vec![TxIn {
                previous_output: OutPoint { txid, vout: 0 },
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

        let unlocking_script = new_collaborative_unlocking_script([public_key1, public_key2]);
        assert_eq!(
            new_collaborative_unlocking_script([public_key1, public_key2]),
            new_collaborative_unlocking_script([public_key2, public_key1]),
        );
        println!("Unlocking Script: {}", unlocking_script);
        let sig_1 = sign_tx_collaborative(
            unsigned_tx.clone(),
            0,
            private_key1,
            multisig_amount,
            unlocking_script.clone(),
        );
        let sig_2 = sign_tx_collaborative(
            unsigned_tx.clone(),
            0,
            private_key2,
            multisig_amount,
            unlocking_script.clone(),
        );
        let signed_tx = combine_signatures(
            unsigned_tx,
            0,
            vec![sig_1, sig_2],
            vec![public_key1, public_key2],
            unlocking_script,
            true,
        );
        assert!(signed_tx.input[0].witness.witness_script().is_some());
        println!(
            "Signed transaction: {}",
            consensus::serialize(&signed_tx).as_hex()
        );
        let result = btc_client.send_raw_transaction(&signed_tx);
        assert!(result.is_ok());
    }
}
