use std::str::FromStr;

use bitcoin::{
    absolute, consensus, hex::DisplayHex, transaction, Address, Amount, OutPoint, Transaction,
    TxIn, TxOut, Txid,
};
use wasm_bindgen::prelude::*;

use crate::{
    scripts::{new_collaborative_address, new_dispute_address},
    util::{convert_network_to_typed, npub_to_public_key},
};

/// Creates a 2-of-2 multisig escrow address for collaboration between two users,
/// and their respective Nostr public keys.
#[wasm_bindgen]
pub fn create_collab_address(npub_1: String, npub_2: String, network: String) -> String {
    let network = convert_network_to_typed(network);
    let public_key_1 = npub_to_public_key(npub_1);
    let public_key_2 = npub_to_public_key(npub_2);
    let address = new_collaborative_address([public_key_1, public_key_2], network);
    address.to_string()
}

/// Creates a 2-of-2/2-of-3 multisig escrow address for collaboration/dispute between two/three users,
/// the timelock duration,
/// and their respective Nostr public keys.
#[wasm_bindgen]
pub fn create_dispute_address(
    npub_1: String,
    npub_2: String,
    npub_arbiter: String,
    timelock_duration: u32,
    network: String,
) -> String {
    let network = convert_network_to_typed(network);
    let public_key_1 = npub_to_public_key(npub_1);
    let public_key_2 = npub_to_public_key(npub_2);
    let public_key_arbiter = npub_to_public_key(npub_arbiter);
    let address = new_dispute_address(
        [public_key_1, public_key_2],
        public_key_arbiter,
        timelock_duration,
        network,
    );
    address.to_string()
}
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
    let pk_1 = npub_to_public_key(npub_1);
    let pk_2 = npub_to_public_key(npub_2);
    let _escrow_address = new_collaborative_address([pk_1, pk_2], network);
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
    let pk_1 = npub_to_public_key(npub_1);
    let pk_2 = npub_to_public_key(npub_2);
    let pk_arbiter = npub_to_public_key(npub_arbiter);
    let _escrow_address = new_dispute_address([pk_1, pk_2], pk_arbiter, timelock_duration, network);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_collab_address() {
        let npub_1 = "npub1lfsec9a40ntx0hjr9wtuchclar7xcyhrf0gngaz3vt5dhnqdndaq099v6c";
        let npub_2 = "npub1ykkf8j4mt0z4hfz5eesqck6a9qcearxq2mlk6f78k3yxhjkpqnxqanyg69";
        let network = "Mainnet";
        let address =
            create_collab_address(npub_1.to_string(), npub_2.to_string(), network.to_string());
        println!("Collaborative Address: {}", address);
    }

    #[test]
    fn test_create_dispute_address() {
        let npub_1 = "npub1lfsec9a40ntx0hjr9wtuchclar7xcyhrf0gngaz3vt5dhnqdndaq099v6c";
        let npub_2 = "npub1ykkf8j4mt0z4hfz5eesqck6a9qcearxq2mlk6f78k3yxhjkpqnxqanyg69";
        let npub_arbiter = "npub1gwpya6nnvsrf6ghkjfu4vt8ccypmqazcupjwtkejzlfwezfye6kqett000";
        let timelock_duration = 100;
        let network = "Mainnet";
        let address = create_dispute_address(
            npub_1.to_string(),
            npub_2.to_string(),
            npub_arbiter.to_string(),
            timelock_duration,
            network.to_string(),
        );
        println!("Dispute Address: {}", address);
    }

    #[test]
    fn test_create_colab_tx() {
        let npub_1 = "npub1lfsec9a40ntx0hjr9wtuchclar7xcyhrf0gngaz3vt5dhnqdndaq099v6c";
        let npub_2 = "npub1ykkf8j4mt0z4hfz5eesqck6a9qcearxq2mlk6f78k3yxhjkpqnxqanyg69";
        let escrow_amount = 100_000_000;
        let resolution_address_1p = "bc1q38tw7nazd0qg8kypv4erk90hqlgwxcu6pn0htk";
        let resolution_address_2p =
            "bc1qtnt0x2qt24qgq5xhtslswp598r9vus38ptyq4nrysf022w8x7dhslv9rna";
        let funding_txid = "602ae1accd9626bde16d19cbe8663cbe37a4e95839d0cddb10b84dcc82f07799";
        let fee = 1000;
        let network = "Mainnet";
        let tx = create_collab_tx(
            npub_1.to_string(),
            npub_2.to_string(),
            escrow_amount,
            resolution_address_1p.to_string(),
            resolution_address_2p.to_string(),
            funding_txid.to_string(),
            fee,
            network.to_string(),
        );
        println!("Collaborative Transaction: {}", tx);
    }

    #[test]
    fn test_dispute_tx() {
        let npub_1 = "npub1lfsec9a40ntx0hjr9wtuchclar7xcyhrf0gngaz3vt5dhnqdndaq099v6c";
        let npub_2 = "npub1ykkf8j4mt0z4hfz5eesqck6a9qcearxq2mlk6f78k3yxhjkpqnxqanyg69";
        let npub_arbiter = "npub1gwpya6nnvsrf6ghkjfu4vt8ccypmqazcupjwtkejzlfwezfye6kqett000";
        let escrow_amount = 100_000_000;
        let resolution_address_1p = "bc1q38tw7nazd0qg8kypv4erk90hqlgwxcu6pn0htk";
        let resolution_address_2p =
            "bc1qtnt0x2qt24qgq5xhtslswp598r9vus38ptyq4nrysf022w8x7dhslv9rna";
        let funding_txid = "602ae1accd9626bde16d19cbe8663cbe37a4e95839d0cddb10b84dcc82f07799";
        let fee = 1000;
        let timelock_duration = 100;
        let network = "Mainnet";
        let tx = create_dispute_tx(
            npub_1.to_string(),
            npub_2.to_string(),
            npub_arbiter.to_string(),
            escrow_amount,
            resolution_address_1p.to_string(),
            resolution_address_2p.to_string(),
            funding_txid.to_string(),
            fee,
            timelock_duration,
            network.to_string(),
        );
        println!("Dispute Transaction: {}", tx);
    }
}
