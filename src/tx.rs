use bitcoin::{
    Address, Amount, Network, OutPoint, Transaction, TxIn, TxOut, Txid, absolute, transaction,
};
use nostr::key::PublicKey as NostPublicKey;

use crate::{
    error::Error,
    scripts::{new_collaborative_address, new_dispute_address},
    util::{npub_to_address, npub_to_public_key},
};

/// Creates a 2-of-2 multisig escrow address for collaboration between two users,
/// and their respective Nostr public keys.
pub fn create_collab_address(
    npub_1: NostPublicKey,
    npub_2: NostPublicKey,
    network: Network,
) -> Result<Address, Error> {
    let public_key_1 = npub_to_public_key(npub_1)?;
    let public_key_2 = npub_to_public_key(npub_2)?;
    Ok(new_collaborative_address(
        [public_key_1, public_key_2],
        network,
    ))
}

/// Creates a 2-of-2/2-of-3 multisig escrow address for collaboration/dispute between two/three users,
/// the timelock duration,
/// and their respective Nostr public keys.
pub fn create_dispute_address(
    npub_1: NostPublicKey,
    npub_2: NostPublicKey,
    npub_arbiter: NostPublicKey,
    timelock_duration: u32,
    network: Network,
) -> Result<Address, Error> {
    let public_key_1 = npub_to_public_key(npub_1)?;
    let public_key_2 = npub_to_public_key(npub_2)?;
    let public_key_arbiter = npub_to_public_key(npub_arbiter)?;
    Ok(new_dispute_address(
        [public_key_1, public_key_2],
        public_key_arbiter,
        timelock_duration,
        network,
    ))
}

/// Creates a 2-of-2/2-of-3 multisig transaction for collaboration/dispute between two/three users,
/// given an escrow amount, and their respective Nostr public keys.
///
/// The user should also specify the funding [`Txid`] that assumes the vout is always 0.
///
/// The resolution address is the address derived from the users' Nostr public key.
///
/// # Errors
///
/// Errors if could not create SegWit-v0 resolution addresses from supplied npubs.
pub fn create_tx(
    npub_1: NostPublicKey,
    npub_2: NostPublicKey,
    escrow_amount_1: Amount,
    escrow_amount_2: Amount,
    funding_txid: Txid,
    fee: Amount,
    network: Network,
) -> Result<Transaction, Error> {
    // Parse stuff
    let prevout = OutPoint {
        txid: funding_txid,
        vout: 0,
    };
    let resolution_address_1 = npub_to_address(npub_1, network)?;
    let resolution_address_2 = npub_to_address(npub_2, network)?;

    // Calculate the fees per participant
    let fees_per_participant = match fee.checked_div(2) {
        Some(fee) => fee,
        None => return Err(Error::Rounding),
    };
    let liquid_escrow_amount_1 = match escrow_amount_1.checked_sub(fees_per_participant) {
        Some(amount) => amount,
        None => return Err(Error::Rounding),
    };
    let liquid_escrow_amount_2 = match escrow_amount_2.checked_sub(fees_per_participant) {
        Some(amount) => amount,
        None => return Err(Error::Rounding),
    };

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
                value: liquid_escrow_amount_1,
                script_pubkey: resolution_address_1.script_pubkey(),
            },
            TxOut {
                value: liquid_escrow_amount_2,
                script_pubkey: resolution_address_2.script_pubkey(),
            },
        ],
    };

    Ok(tx)
}

#[cfg(test)]
mod tests {
    use bitcoin::{consensus, hex::DisplayHex};

    use crate::util::parse_npub;

    use super::*;

    #[test]
    fn test_create_collab_address() {
        let npub_1 = parse_npub(
            "npub1lfsec9a40ntx0hjr9wtuchclar7xcyhrf0gngaz3vt5dhnqdndaq099v6c".to_string(),
        )
        .unwrap();
        let npub_2 = parse_npub(
            "npub1ykkf8j4mt0z4hfz5eesqck6a9qcearxq2mlk6f78k3yxhjkpqnxqanyg69".to_string(),
        )
        .unwrap();
        let network = Network::Bitcoin;
        let address = create_collab_address(npub_1, npub_2, network).unwrap();
        let expected = "bc1qveh7msk0kwwjyd7vglswhpqg4nlqaka8svexqfuvl3fd7qgludfskcsrqp";
        assert_eq!(address.to_string(), expected);
    }

    #[test]
    fn test_create_dispute_address() {
        let npub_1 = parse_npub(
            "npub1lfsec9a40ntx0hjr9wtuchclar7xcyhrf0gngaz3vt5dhnqdndaq099v6c".to_string(),
        )
        .unwrap();
        let npub_2 = parse_npub(
            "npub1ykkf8j4mt0z4hfz5eesqck6a9qcearxq2mlk6f78k3yxhjkpqnxqanyg69".to_string(),
        )
        .unwrap();
        let npub_arbiter = parse_npub(
            "npub1gwpya6nnvsrf6ghkjfu4vt8ccypmqazcupjwtkejzlfwezfye6kqett000".to_string(),
        )
        .unwrap();
        let timelock_duration = 100;
        let network = Network::Bitcoin;
        let address =
            create_dispute_address(npub_1, npub_2, npub_arbiter, timelock_duration, network)
                .unwrap();
        let expected = "bc1q2mq7655gjphmx2tptyszcf3vhg4y3n5488rzw30gs26uumc6gtcsxgn2ls";
        assert_eq!(address.to_string(), expected);
    }

    #[test]
    fn test_create_tx() {
        let npub_1 = parse_npub(
            "npub1lfsec9a40ntx0hjr9wtuchclar7xcyhrf0gngaz3vt5dhnqdndaq099v6c".to_string(),
        )
        .unwrap();
        let npub_2 = parse_npub(
            "npub1ykkf8j4mt0z4hfz5eesqck6a9qcearxq2mlk6f78k3yxhjkpqnxqanyg69".to_string(),
        )
        .unwrap();
        let escrow_amount_1 = Amount::from_sat(50_000_000);
        let escrow_amount_2 = Amount::from_sat(50_000_000);
        let funding_txid = "602ae1accd9626bde16d19cbe8663cbe37a4e95839d0cddb10b84dcc82f07799"
            .parse::<Txid>()
            .unwrap();
        let fee = Amount::from_sat(1_000);
        let network = Network::Bitcoin;
        let tx = create_tx(
            npub_1,
            npub_2,
            escrow_amount_1,
            escrow_amount_2,
            funding_txid,
            fee,
            network,
        )
        .unwrap();
        println!(
            "Collaborative Transaction: {}",
            consensus::serialize(&tx).as_hex().to_string()
        );

        let resolution_address_1p = npub_to_address(npub_1, network).unwrap();
        let resolution_address_2p = npub_to_address(npub_2, network).unwrap();

        assert_eq!(
            tx.output[0].script_pubkey,
            resolution_address_1p.script_pubkey()
        );
        assert_eq!(
            tx.output[1].script_pubkey,
            resolution_address_2p.script_pubkey()
        );
    }
}
