use serde::Deserialize;
use wasm_bindgen::prelude::*;
use core::net;
use std::{collections::BTreeMap, str::FromStr};

use bitcoin::{
    consensus, ecdsa, hex::{DisplayHex, FromHex}, sighash::SighashCache, Amount, EcdsaSighashType, PrivateKey, Psbt, PublicKey, ScriptBuf, Transaction, TxOut
};
use secp256k1::{Message, SECP256K1};

use crate::{
    scripts::{new_collaborative_address, new_dispute_address}, sign::{combine_signatures_collaborative, combine_signatures_dispute_arbitrator, combine_signatures_dispute_collaborative, sign_tx}, tx, util::{self, convert_network_to_typed, convert_npub_to_public_key}
};

/// Signs a [`Transaction`] input `index` using a [`PrivateKey`].
///
/// The input is signed using the provided [`PrivateKey`], [`Amount`], and [`ScriptBuf`] locking script.
#[wasm_bindgen]
pub fn sign_tx_wasm(
    tx: String,
    index: usize,
    nsec: String,
    amount: u64,
    unlocking_script: String,
    network: String,
) -> String {
    let private_key = convert_nsec_to_secret_key_wasm(nsec, network);
    let amount = Amount::from_sat(amount);
    let unlocking_script: ScriptBuf = ScriptBuf::from_bytes(unlocking_script.as_bytes().to_vec());
    let tx = consensus::deserialize(&tx.into_bytes()).unwrap();
    let signature = sign_tx(tx, index, private_key, amount, unlocking_script);
    signature.serialize().as_hex().to_string()
}

pub fn convert_nsec_to_secret_key_wasm(nsec: String, network: String) -> PrivateKey {
    let network = convert_network_to_typed(network);
    util::convert_nsec_to_secret_key(nsec, network)
}

#[wasm_bindgen]
pub fn combine_signatures_collaborative_wasm(
    tx: String,
    index: usize,
    signatures: Vec<String>,
    pks: Vec<String>,
    unlocking_script: String,
) -> String {
    let tx: Transaction = consensus::deserialize(&tx.into_bytes()).unwrap();
    let signatures: Vec<ecdsa::Signature> = signatures.into_iter().map(|sig| ecdsa::Signature::from_str(&sig).unwrap()).collect();
    let pks: Vec<PublicKey> = pks.into_iter().map(|pk| PublicKey::from_slice(&pk.into_bytes()).unwrap()).collect();
    let unlocking_script: ScriptBuf = ScriptBuf::from_bytes(unlocking_script.as_bytes().to_vec());
    let tx: Transaction = combine_signatures_collaborative(tx, index, signatures, pks, unlocking_script);
    consensus::serialize(&tx).as_hex().to_string()
}

#[wasm_bindgen]
pub fn combine_signatures_dispute_collaborative_wasm(
    tx: String,
    index: usize,
    signatures: Vec<String>,
    pks: Vec<String>,
    unlocking_script: String,
) -> String {
    let tx: Transaction = consensus::deserialize(&tx.into_bytes()).unwrap();
    let signatures: Vec<ecdsa::Signature> = signatures.into_iter().map(|sig| ecdsa::Signature::from_str(&sig).unwrap()).collect();
    let pks: Vec<PublicKey> = pks.into_iter().map(|pk| PublicKey::from_slice(&pk.into_bytes()).unwrap()).collect();
    let unlocking_script: ScriptBuf = ScriptBuf::from_bytes(unlocking_script.as_bytes().to_vec());
    let tx: Transaction = combine_signatures_dispute_collaborative(tx, index, signatures, pks, unlocking_script);
    consensus::serialize(&tx).as_hex().to_string()
}

#[wasm_bindgen]
pub fn combine_signatures_dispute_arbitrator_wasm(
    tx: String,
    index: usize,
    signatures: Vec<String>,
    pks: Vec<String>,
    unlocking_script: String,
) -> String {
    let tx: Transaction = consensus::deserialize(&tx.into_bytes()).unwrap();
    let signatures: Vec<ecdsa::Signature> = signatures.into_iter().map(|sig| ecdsa::Signature::from_str(&sig).unwrap()).collect();
    let pks: Vec<PublicKey> = pks.into_iter().map(|pk| PublicKey::from_slice(&pk.into_bytes()).unwrap()).collect();
    let unlocking_script: ScriptBuf = ScriptBuf::from_bytes(unlocking_script.as_bytes().to_vec());
    let tx: Transaction = combine_signatures_dispute_arbitrator(tx, index, signatures, pks, unlocking_script);
    consensus::serialize(&tx).as_hex().to_string()
}

