use std::str::FromStr;

use bitcoin::{
    absolute, consensus, hex::DisplayHex, transaction, Address, Amount, OutPoint, Transaction,
    TxIn, TxOut, Txid,
};
use wasm_bindgen::prelude::*;

use crate::{
    scripts::{new_collaborative_address, new_dispute_address},
    util::{convert_network_to_typed, convert_npub_to_public_key},
};

/// Creates a 2-of-2 multisig transaction for collaboration between two users, given an escrow amount,
/// their respective Nostr public keys; and their respective resolution addresses.
///
/// The user should also specify the funding [`Txid`] that assumes the vout is always 0.
#[allow(clippy::too_many_arguments)]
#[wasm_bindgen]
pub fn create_collab_tx(
    npub_1: String,
    npub_2: String,
    escrow_amount: u64,
    resolution_address_1p: String,
    resolution_address_2p: String,
    funding_txid: String,
    fee: u64,
    network: String,
) -> String {
    // Parse stuff
    let network = convert_network_to_typed(network);
    let txid = Txid::from_str(&funding_txid).unwrap();
    let resolution_address_1p = Address::from_str(&resolution_address_1p)
        .unwrap()
        .require_network(network)
        .unwrap();
    let resolution_address_2p = Address::from_str(&resolution_address_2p)
        .unwrap()
        .require_network(network)
        .unwrap();
    let prevout = OutPoint { txid, vout: 0 };
    let pk_1 = convert_npub_to_public_key(npub_1);
    let pk_2 = convert_npub_to_public_key(npub_2);
    let escrow_address = new_collaborative_address([pk_1, pk_2], network);
    let escrow_amount = Amount::from_sat(escrow_amount);
    let fee = Amount::from_sat(fee);

    // Calculate the values
    // 1P will be rounded up and 2P will be rounded down
    // before the fee
    let liquid_amount = escrow_amount - fee;
    let liquid_amount_1p = liquid_amount.checked_div(3).unwrap();
    let liquid_amount_2p = liquid_amount
        .checked_div(3)
        .unwrap()
        .checked_mul(2)
        .unwrap();

    // Create the transaction
    let tx = Transaction {
        version: transaction::Version(2),
        lock_time: absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: prevout,
            ..Default::default()
        }],
        output: vec![
            TxOut {
                value: escrow_amount,
                script_pubkey: escrow_address.script_pubkey(),
            },
            TxOut {
                value: liquid_amount_1p,
                script_pubkey: resolution_address_1p.script_pubkey(),
            },
            TxOut {
                value: liquid_amount_2p,
                script_pubkey: resolution_address_2p.script_pubkey(),
            },
        ],
    };

    consensus::encode::serialize(&tx).as_hex().to_string()
}

/// Creates a 2-of-2/2-of-3 multisig transaction for collaboration/dispute between two/three users,
/// given an escrow amount, their respective Nostr public keys;
/// and their respective resolution addresses.
///
/// The user should also specify the funding [`Txid`] that assumes the vout is always 0.
#[allow(clippy::too_many_arguments)]
#[wasm_bindgen]
pub fn create_dispute_tx(
    npub_1: String,
    npub_2: String,
    npub_arbiter: String,
    escrow_amount: u64,
    resolution_address_1p: String,
    resolution_address_2p: String,
    funding_txid: String,
    fee: u64,
    timelock_duration: u32,
    network: String,
) -> String {
    // Parse stuff
    let network = convert_network_to_typed(network);
    let txid = Txid::from_str(&funding_txid).unwrap();
    let resolution_address_1p = Address::from_str(&resolution_address_1p)
        .unwrap()
        .require_network(network)
        .unwrap();
    let resolution_address_2p = Address::from_str(&resolution_address_2p)
        .unwrap()
        .require_network(network)
        .unwrap();
    let prevout = OutPoint { txid, vout: 0 };
    let pk_1 = convert_npub_to_public_key(npub_1);
    let pk_2 = convert_npub_to_public_key(npub_2);
    let pk_arbiter = convert_npub_to_public_key(npub_arbiter);
    let escrow_address = new_dispute_address([pk_1, pk_2], pk_arbiter, timelock_duration, network);
    let escrow_amount = Amount::from_sat(escrow_amount);
    let fee = Amount::from_sat(fee);

    // Calculate the values
    // 1P will be rounded up and 2P will be rounded down
    // before the fee
    let liquid_amount = escrow_amount - fee;
    let liquid_amount_1p = liquid_amount.checked_div(3).unwrap();
    let liquid_amount_2p = liquid_amount
        .checked_div(3)
        .unwrap()
        .checked_mul(2)
        .unwrap();

    // Create the transaction
    let tx = Transaction {
        version: transaction::Version(2),
        lock_time: absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: prevout,
            ..Default::default()
        }],
        output: vec![
            TxOut {
                value: escrow_amount,
                script_pubkey: escrow_address.script_pubkey(),
            },
            TxOut {
                value: liquid_amount_1p,
                script_pubkey: resolution_address_1p.script_pubkey(),
            },
            TxOut {
                value: liquid_amount_2p,
                script_pubkey: resolution_address_2p.script_pubkey(),
            },
        ],
    };

    consensus::encode::serialize(&tx).as_hex().to_string()
}
