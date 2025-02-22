use std::str::FromStr;
use wasm_bindgen::prelude::*;

use crate::{
    sign::{
        combine_signatures_collaborative, combine_signatures_dispute_arbitrator,
        combine_signatures_dispute_collaborative, sign_tx,
    },
    util::{self, convert_network_to_typed},
};
use bitcoin::{consensus, ecdsa, hex::DisplayHex, Amount, PrivateKey, ScriptBuf, Transaction};

/// This module provides functionality for signing transactions and combining signatures
/// in various collaborative and dispute scenarios.
///
/// The following functions are imported from other modules:
///
/// - `combine_signatures_collaborative`: Combines signatures in a collaborative scenario.
/// - `combine_signatures_dispute_arbitrator`: Combines signatures in a dispute scenario with an arbitrator.
/// - `combine_signatures_dispute_collaborative`: Combines signatures in a dispute scenario collaboratively.
/// - `sign_tx`: Signs a transaction.
///
/// Additionally, utility functions and types are imported from the `util` module, including:
///
/// - `convert_network_to_typed`: Converts a network to a typed representation.

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
    npubs: Vec<String>,
    unlocking_script: String,
) -> String {
    let tx: Transaction = consensus::deserialize(&tx.into_bytes()).unwrap();
    let signatures: Vec<ecdsa::Signature> = signatures
        .into_iter()
        .map(|sig| ecdsa::Signature::from_str(&sig).unwrap())
        .collect();
    let pks = npubs
        .into_iter()
        .map(util::convert_npub_to_public_key)
        .collect();
    let unlocking_script: ScriptBuf = ScriptBuf::from_bytes(unlocking_script.as_bytes().to_vec());
    let tx: Transaction =
        combine_signatures_collaborative(tx, index, signatures, pks, unlocking_script);
    consensus::serialize(&tx).as_hex().to_string()
}

#[wasm_bindgen]
pub fn combine_signatures_dispute_collaborative_wasm(
    tx: String,
    index: usize,
    signatures: Vec<String>,
    npubs: Vec<String>,
    unlocking_script: String,
) -> String {
    let tx: Transaction = consensus::deserialize(&tx.into_bytes()).unwrap();
    let signatures: Vec<ecdsa::Signature> = signatures
        .into_iter()
        .map(|sig| ecdsa::Signature::from_str(&sig).unwrap())
        .collect();
    let pks = npubs
        .into_iter()
        .map(util::convert_npub_to_public_key)
        .collect();
    let unlocking_script: ScriptBuf = ScriptBuf::from_bytes(unlocking_script.as_bytes().to_vec());
    let tx: Transaction =
        combine_signatures_dispute_collaborative(tx, index, signatures, pks, unlocking_script);
    consensus::serialize(&tx).as_hex().to_string()
}

#[wasm_bindgen]
pub fn combine_signatures_dispute_arbitrator_wasm(
    tx: String,
    index: usize,
    signatures: Vec<String>,
    npubs: Vec<String>,
    unlocking_script: String,
) -> String {
    let tx: Transaction = consensus::deserialize(&tx.into_bytes()).unwrap();
    let signatures: Vec<ecdsa::Signature> = signatures
        .into_iter()
        .map(|sig| ecdsa::Signature::from_str(&sig).unwrap())
        .collect();
    let pks = npubs
        .into_iter()
        .map(util::convert_npub_to_public_key)
        .collect();
    let unlocking_script: ScriptBuf = ScriptBuf::from_bytes(unlocking_script.as_bytes().to_vec());
    let tx: Transaction =
        combine_signatures_dispute_arbitrator(tx, index, signatures, pks, unlocking_script);
    consensus::serialize(&tx).as_hex().to_string()
}
