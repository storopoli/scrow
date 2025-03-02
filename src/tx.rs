//! Creates Taproot Transactions using Nostr keys.

use bitcoin::{
    Address, Amount, Network, OutPoint, Sequence, Transaction, TxIn, TxOut, Txid, absolute,
    transaction,
};
#[cfg(debug_assertions)]
use dioxus::logger::tracing::trace;
use nostr::key::PublicKey as NostPublicKey;

use crate::{error::Error, util::npub_to_address};

/// Creates a [`Transaction`] that swipe the resolution address to a `destination` [`Address`].
///
/// Assumes that the resolution address is derived from the users' Nostr public key
/// and has received a single input.
pub(crate) fn resolution_tx(
    amount: Amount,
    funding_txid: Txid,
    destination: &Address,
    fee: Amount,
) -> Transaction {
    // Parse stuff
    let prevout = OutPoint {
        txid: funding_txid,
        vout: 0,
    };

    // Create the transaction
    Transaction {
        version: transaction::Version(2),
        lock_time: absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: prevout,
            ..Default::default()
        }],
        output: vec![TxOut {
            value: amount - fee,
            script_pubkey: destination.script_pubkey(),
        }],
    }
}

/// Creates a 2-of-2/2-of-3 multisig [`Transaction`] for collaboration/dispute between two/three users,
/// given an escrow amount, and their respective Nostr public keys (`npub`s).
///
/// The user should also specify the funding [`Txid`] that assumes the vout is always 0.
///
/// The resolution address is the address derived from the users' `npub`s.
///
/// # Errors
///
/// Errors if could not create SegWit-v1 P2TR resolution addresses from supplied `npub`s.
#[expect(clippy::too_many_arguments)]
pub(crate) fn escrow_tx(
    npub_1: &NostPublicKey,
    npub_2: &NostPublicKey,
    timelock_duration: Option<u32>,
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
    #[cfg(debug_assertions)]
    trace!(%fees_per_participant, "fees per participant");
    let liquid_escrow_amount_1 = match escrow_amount_1.checked_sub(fees_per_participant) {
        Some(amount) => amount,
        None => return Err(Error::Rounding),
    };
    #[cfg(debug_assertions)]
    trace!(%liquid_escrow_amount_1, "liquid escrow amount 1");
    let liquid_escrow_amount_2 = match escrow_amount_2.checked_sub(fees_per_participant) {
        Some(amount) => amount,
        None => return Err(Error::Rounding),
    };
    #[cfg(debug_assertions)]
    trace!(%liquid_escrow_amount_2, "liquid escrow amount 2");

    let timelock_duration = timelock_duration.unwrap_or_default();
    #[cfg(debug_assertions)]
    trace!(%timelock_duration, "timelock duration");

    // Create the transaction
    let tx = Transaction {
        version: transaction::Version(2),
        lock_time: absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: prevout,
            sequence: Sequence::from_consensus(timelock_duration),
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
    fn test_escrow_tx() {
        let npub_1 =
            parse_npub("npub1lfsec9a40ntx0hjr9wtuchclar7xcyhrf0gngaz3vt5dhnqdndaq099v6c").unwrap();
        let npub_2 =
            parse_npub("npub1ykkf8j4mt0z4hfz5eesqck6a9qcearxq2mlk6f78k3yxhjkpqnxqanyg69").unwrap();
        let escrow_amount_1 = Amount::from_sat(50_000_000);
        let escrow_amount_2 = Amount::from_sat(50_000_000);
        let funding_txid = "602ae1accd9626bde16d19cbe8663cbe37a4e95839d0cddb10b84dcc82f07799"
            .parse::<Txid>()
            .unwrap();
        let fee = Amount::from_sat(1_000);
        let network = Network::Bitcoin;
        let tx = escrow_tx(
            &npub_1,
            &npub_2,
            None,
            escrow_amount_1,
            escrow_amount_2,
            funding_txid,
            fee,
            network,
        )
        .unwrap();
        println!(
            "Collaborative Transaction: {}",
            consensus::serialize(&tx).as_hex()
        );

        let resolution_address_1p = npub_to_address(&npub_1, network).unwrap();
        let resolution_address_2p = npub_to_address(&npub_2, network).unwrap();

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
