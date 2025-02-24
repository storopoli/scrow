use bitcoin::{
    ecdsa, sighash::SighashCache, Amount, EcdsaSighashType, PrivateKey, PublicKey, ScriptBuf,
    Transaction,
};
use secp256k1::{Message, SECP256K1};

/// Signs a [`Transaction`] input `index` using a [`PrivateKey`].
///
/// The input is signed using the provided [`PrivateKey`], [`Amount`], and [`ScriptBuf`] locking script.
pub fn sign_tx(
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
/// Only use this of the 2-of-2 multisig collaborative.
///
/// It also sorts the PKs to see which one is the first to go into the witness stack,
/// given a sorted multisig script.
pub fn combine_signatures_collaborative(
    tx: Transaction,
    index: usize,
    signatures: Vec<ecdsa::Signature>,
    pks: Vec<PublicKey>,
    unlocking_script: ScriptBuf,
) -> Transaction {
    let mut transaction = tx;

    // Collaborative means 2-of-2 multisig.
    // And we need a fucking empty thing first.
    transaction.input[index].witness.push([]);

    // Create pairs of PKs and signatures and sort by PK
    let mut pairs: Vec<(PublicKey, ecdsa::Signature)> = pks.into_iter().zip(signatures).collect();

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

/// Combine multiple [`ecdsa::Signature`]s into a single [`Transaction`] input.
///
/// Only use this of the 2-of-3 multisig dispute in a collaborative path.
///
/// It also sorts the PKs to see which one is the first to go into the witness stack.
pub fn combine_signatures_dispute_collaborative(
    tx: Transaction,
    index: usize,
    signatures: Vec<ecdsa::Signature>,
    pks: Vec<PublicKey>,
    unlocking_script: ScriptBuf,
) -> Transaction {
    let mut transaction = tx;

    // Collaborative means 2-of-2 multisig.
    // And we need a fucking empty thing first.
    transaction.input[index].witness.push([]);

    // Create pairs of PKs and signatures and sort by PK
    let mut pairs: Vec<(PublicKey, ecdsa::Signature)> = pks.into_iter().zip(signatures).collect();

    // Sort pairs based on public keys
    pairs.sort_by(|a, b| a.0.cmp(&b.0));

    // Push signatures in order of sorted public keys
    for (_, signature) in pairs {
        transaction.input[index].witness.push(signature.serialize());
    }

    // Push true to activate OP_IF
    transaction.input[index].witness.push([0x01]);

    // Finally, push the unlocking script
    transaction.input[index].witness.push(&unlocking_script);
    transaction
}

/// Combine multiple [`ecdsa::Signature`]s into a single [`Transaction`] input.
///
/// Only use this of the 2-of-3 multisig dispute in a arbitrator path.
///
/// It also sorts the PKs to see which one is the first to go into the witness stack,
/// given a sorted multisig script.
pub fn combine_signatures_dispute_arbitrator(
    tx: Transaction,
    index: usize,
    signatures: Vec<ecdsa::Signature>,
    pks: Vec<PublicKey>,
    unlocking_script: ScriptBuf,
) -> Transaction {
    let mut transaction = tx;

    // 2-of-3 multisig.
    // We need a fucking empty thing first.
    transaction.input[index].witness.push([]);

    // Create pairs of PKs and signatures and sort by PK
    let mut pairs: Vec<(PublicKey, ecdsa::Signature)> = pks.into_iter().zip(signatures).collect();

    // Sort pairs based on public keys
    pairs.sort_by(|a, b| a.0.cmp(&b.0));

    // Push signatures in order of sorted public keys
    for (_, signature) in pairs {
        transaction.input[index].witness.push(signature.serialize());
    }

    // Push false to activate OP_ELSE
    transaction.input[index].witness.push([]);

    // Finally, push the unlocking script
    transaction.input[index].witness.push(&unlocking_script);
    transaction
}

#[cfg(test)]
mod tests {
    use bitcoin::{
        absolute::LockTime, consensus, ecdsa, hex::DisplayHex, sighash::SighashCache, transaction,
        Address, Amount, BlockHash, CompressedPublicKey, Network, OutPoint, Sequence, Transaction,
        TxIn, TxOut, Witness,
    };
    use corepc_node::Node;
    use secp256k1::{Message, SecretKey};

    use crate::scripts::{
        new_collaborative_address, new_collaborative_unlocking_script, new_dispute_address,
        new_dispute_unlocking_script,
    };

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
    const PRIVATE_KEY3_HEX: &str =
        "7c77a37bb1d16c5a0eeb6ce8ea6cd06850400473241ab65f3adbf29a1b822b75";
    // NOTE: it is uncompressed prepended with 0x02 for the Even parity.
    const PUBLIC_KEY3_HEX: &str =
        "032d7b3d8028c474251676708ec41f12100685b200ccbb394e5e782d73b233a8eb";

    #[test]
    fn sign_p2wsh_collaborative_tx_flow() {
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
        let sig_1 = sign_tx(
            unsigned_tx.clone(),
            0,
            private_key1,
            multisig_amount,
            unlocking_script.clone(),
        );
        let sig_2 = sign_tx(
            unsigned_tx.clone(),
            0,
            private_key2,
            multisig_amount,
            unlocking_script.clone(),
        );
        let signed_tx = combine_signatures_collaborative(
            unsigned_tx,
            0,
            vec![sig_1, sig_2],
            vec![public_key1, public_key2],
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
        let sig_2 = sign_tx(
            unsigned_tx.clone(),
            0,
            private_key2,
            multisig_amount,
            unlocking_script.clone(),
        );
        let signed_tx = combine_signatures_dispute_collaborative(
            unsigned_tx,
            0,
            vec![sig_1, sig_2],
            vec![public_key1, public_key2],
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
}
