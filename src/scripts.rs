//! Creates Tapscripts using Nostr keys.

use std::sync::LazyLock;

use bitcoin::{
    Address, Network, ScriptBuf, Sequence, XOnlyPublicKey,
    hashes::{Hash, sha256},
    opcodes::all::*,
    taproot::{LeafVersion, TaprootBuilder, TaprootBuilderError, TaprootSpendInfo},
};
#[cfg(debug_assertions)]
use dioxus::logger::tracing::trace;
use nostr::key::PublicKey as NostrPublicKey;
use secp256k1::SECP256K1;

use crate::{error::Error, util::npub_to_x_only_public_key};

/// A verifiably unspendable public key, produced by hashing a fixed string to a curve group
/// generator.
///
/// This is related to the technique used in
/// [BIP-341](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki#constructing-and-spending-taproot-outputs).
///
/// Note that this is _not_ necessarily a uniformly-sampled curve point!
///
/// But this is fine; we only need a generator with no efficiently-computable discrete logarithm
/// relation against the standard generator.
pub const UNSPENDABLE_PUBLIC_KEY_INPUT: &[u8] = b"X-only-PK unspendable";
pub static UNSPENDABLE_PUBLIC_KEY: LazyLock<XOnlyPublicKey> = LazyLock::new(|| {
    XOnlyPublicKey::from_slice(sha256::Hash::hash(UNSPENDABLE_PUBLIC_KEY_INPUT).as_byte_array())
        .expect("valid xonly public key")
});

/// Creates an escrow-resolution 2-of-3 multisig P2TR [`TaprootSpendInfo`] from 2 [`NostrPublicKey`]s,
/// an optional arbitrator [`NostrPublicKey`] and an optional timelock duration in blocks.
///
/// # Spending Conditions
///
/// - 2-of-2 multisig between the two parties without timelocks.
/// - 2-of-3 multisig between the one of the parties and the arbitrator with a timelock
///   (if using an arbitrator).
///
/// # Merkle Tree Layout
///
/// 1. `A`: 2-of-2 multisig between the two parties without timelocks.
/// 2. `B`: 2-of-3 multisig between the first of the parties and the arbitrator with a timelock
///    (if using an arbitrator).
/// 3. `C`: 2-of-3 multisig between the second of the parties and the arbitrator with a timelock
///    (if using an arbitrator).
///
/// `A` is at depth 1, and `B` and `C` are at depth 2.
///
/// ```text
///     root
///        \
///        /\
///       /  \
///      A    *
///          / \
///         /   \
///        B     C
/// ```
pub fn escrow_spend_info(
    npub_1: &NostrPublicKey,
    npub_2: &NostrPublicKey,
    npub_arbitrator: Option<&NostrPublicKey>,
    timelock_duration: Option<u32>,
) -> Result<TaprootSpendInfo, Error> {
    // Collaborative Path
    if npub_arbitrator.is_none() && timelock_duration.is_none() {
        #[cfg(debug_assertions)]
        trace!("Collaborative path");

        let script_1 = escrow_scripts(
            npub_1,
            npub_2,
            npub_arbitrator,
            timelock_duration,
            EscrowScript::A,
        )?;

        TaprootBuilder::new()
            .add_leaf_with_ver(0, script_1, LeafVersion::TapScript)?
            .finalize(SECP256K1, *UNSPENDABLE_PUBLIC_KEY)
            // FIXME(@storopoli): better error here.
            .map_err(|_| Error::TaprootBuilder(TaprootBuilderError::EmptyTree))
    }
    // Arbitrator path.
    else if npub_arbitrator.is_some() && timelock_duration.is_some() {
        #[cfg(debug_assertions)]
        trace!(npub_arbitrator = %npub_arbitrator.unwrap().to_hex(), timelock_duration = %timelock_duration.unwrap(), "Arbitrator path");

        let script_1 = escrow_scripts(
            npub_1,
            npub_2,
            npub_arbitrator,
            timelock_duration,
            EscrowScript::A,
        )?;

        let script_2 = escrow_scripts(
            npub_1,
            npub_2,
            npub_arbitrator,
            timelock_duration,
            EscrowScript::B,
        )?;
        let script_3 = escrow_scripts(
            npub_1,
            npub_2,
            npub_arbitrator,
            timelock_duration,
            EscrowScript::C,
        )?;

        TaprootBuilder::new()
            .add_leaf_with_ver(1, script_1, LeafVersion::TapScript)?
            .add_leaf_with_ver(2, script_2, LeafVersion::TapScript)?
            .add_leaf_with_ver(2, script_3, LeafVersion::TapScript)?
            .finalize(SECP256K1, *UNSPENDABLE_PUBLIC_KEY)
            // FIXME(@storopoli): better error here.
            .map_err(|_| Error::TaprootBuilder(TaprootBuilderError::EmptyTree))
    }
    // If the match arm failed, means that the inputs to this functions are wrong.
    else {
        Err(Error::WrongInputs(format!(
            "Wrong inputs. Either pass npub_arbitrator and timelock_duration as Some or None. Got npub_arbitrator: {npub_arbitrator:?}. Got timelock_duration: {timelock_duration:?}"
        )))
    }
}

/// Creates an escrow-resolution 2-of-3 multisig P2TR [`TaprootSpendInfo`] from 2 [`NostrPublicKey`]s,
/// an optional arbitrator [`NostrPublicKey`] and an optional timelock duration in blocks.
///
/// # Spending Conditions
///
/// - 2-of-2 multisig between the two parties without timelocks.
/// - 2-of-3 multisig between the one of the parties and the arbitrator with a timelock
///   (if using an arbitrator).
///
/// # Merkle Tree Layout
///
/// 1. `A`: 2-of-2 multisig between the two parties without timelocks.
/// 2. `B`: 2-of-3 multisig between the first of the parties and the arbitrator with a timelock
///    (if using an arbitrator).
/// 3. `C`: 2-of-3 multisig between the second of the parties and the arbitrator with a timelock
///    (if using an arbitrator).
///
/// `A` is at depth 1, and `B` and `C` are at depth 2.
///
/// ```text
///     root
///        \
///        /\
///       /  \
///      A    *
///          / \
///         /   \
///        B     C
/// ```
pub fn escrow_scripts(
    npub_1: &NostrPublicKey,
    npub_2: &NostrPublicKey,
    npub_arbitrator: Option<&NostrPublicKey>,
    timelock_duration: Option<u32>,
    escrow_script: EscrowScript,
) -> Result<ScriptBuf, Error> {
    // Parse npubs to bitcoin public keys.
    let pk_1 = npub_to_x_only_public_key(npub_1)?;
    let pk_2 = npub_to_x_only_public_key(npub_2)?;

    match escrow_script {
        EscrowScript::A => Ok(ScriptBuf::builder()
            // The script should be constructed so the first signature checked is pk2 (last in stack)
            // and the second signature checked is pk1 (second-to-last in stack)
            .push_x_only_key(&pk_2)
            .push_opcode(OP_CHECKSIGVERIFY)
            .push_x_only_key(&pk_1)
            .push_opcode(OP_CHECKSIG)
            .into_script()),
        EscrowScript::B => {
            let npub_arbitrator = npub_arbitrator.unwrap();
            let pk_arbitrator = npub_to_x_only_public_key(npub_arbitrator)?;
            // Timelock.
            let sequence = Sequence::from_consensus(timelock_duration.unwrap());
            Ok(ScriptBuf::builder()
                .push_sequence(sequence)
                .push_opcode(OP_CSV)
                .push_opcode(OP_DROP)
                .push_x_only_key(&pk_arbitrator)
                .push_opcode(OP_CHECKSIGVERIFY)
                .push_x_only_key(&pk_1)
                .push_opcode(OP_CHECKSIG)
                .into_script())
        }
        EscrowScript::C => {
            let npub_arbitrator = npub_arbitrator.unwrap();
            let pk_arbitrator = npub_to_x_only_public_key(npub_arbitrator)?;
            // Timelock.
            let sequence = Sequence::from_consensus(timelock_duration.unwrap());
            Ok(ScriptBuf::builder()
                .push_sequence(sequence)
                .push_opcode(OP_CSV)
                .push_opcode(OP_DROP)
                .push_x_only_key(&pk_arbitrator)
                .push_opcode(OP_CHECKSIGVERIFY)
                .push_x_only_key(&pk_2)
                .push_opcode(OP_CHECKSIG)
                .into_script())
        }
    }
}

/// The escrow script type.
///
/// 1. `A`: 2-of-2 multisig between the two parties without timelocks.
/// 2. `B`: 2-of-3 multisig between the first of the parties and the arbitrator with a timelock
///    (if using an arbitrator).
/// 3. `C`: 2-of-3 multisig between the second of the parties and the arbitrator with a timelock
///    (if using an arbitrator).
///
/// `A` is at depth 1, and `B` and `C` are at depth 2.
///
/// ```text
///     root
///        \
///        /\
///       /  \
///      A    *
///          / \
///         /   \
///        B     C
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EscrowScript {
    A,
    B,
    C,
}

/// Creates an escrow-resolution 2-of-3 multisig P2TR [`Address`] from 2 [`NostrPublicKey`]s,
/// an optional arbitrator [`NostrPublicKey`] and an optional timelock duration in blocks.
///
/// # Spending Conditions
///
/// - 2-of-2 multisig between the two parties without timelocks.
/// - 2-of-3 multisig between the one of the parties and the arbitrator with a timelock
///   (if using an arbitrator).
///
/// # Merkle Tree Layout
///
/// 1. `A`: 2-of-2 multisig between the two parties without timelocks.
/// 2. `B`: 2-of-3 multisig between the first of the parties and the arbitrator with a timelock
///    (if using an arbitrator).
/// 3. `C`: 2-of-3 multisig between the second of the parties and the arbitrator with a timelock
///    (if using an arbitrator).
///
/// `A` is at depth 1, and `B` and `C` are at depth 2.
///
/// ```text
///     root
///        \
///        /\
///       /  \
///      A    *
///          / \
///         /   \
///        B     C
/// ```
pub fn escrow_address(
    npub_1: &NostrPublicKey,
    npub_2: &NostrPublicKey,
    npub_arbitrator: Option<&NostrPublicKey>,
    timelock_duration: Option<u32>,
    network: Network,
) -> Result<Address, Error> {
    let taproot_spend_info = escrow_spend_info(npub_1, npub_2, npub_arbitrator, timelock_duration)?;

    let internal_key = taproot_spend_info.internal_key();
    let merkle_root = taproot_spend_info.merkle_root();

    Ok(Address::p2tr(SECP256K1, internal_key, merkle_root, network))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use bitcoin::AddressType;

    use super::*;

    // Taken from https://docs.rs/bitcoin/latest/bitcoin/struct.PublicKey.html
    const KEY_A: &str = "8f47dcd43ba6d97fc9ed2e3bba09b175a45fac55f0683e8cf771e8ced4572354";
    const KEY_B: &str = "8bde91b10013e08949a318018fedbd896534a549a278e220169ee2a36517c7aa";
    const KEY_C: &str = "2b8324c93575034047a52e9bca05a46d8347046b91a032eff07d5de8d3f2730b";

    #[test]
    fn test_unspendable() {
        // Check that construction of the unspendable key succeeds
        let _ = *UNSPENDABLE_PUBLIC_KEY;
    }

    #[test]
    fn collaborative_address() {
        let npub_1 = NostrPublicKey::from_str(KEY_A).unwrap();
        let npub_2 = NostrPublicKey::from_str(KEY_B).unwrap();
        let network = Network::Testnet;

        let address = escrow_address(&npub_1, &npub_2, None, None, network).unwrap();

        assert_eq!(address.address_type().unwrap(), AddressType::P2tr);
        assert_eq!(
            address.to_string(),
            "tb1pw9lk5k85v58rn2s8ccdxcp62khvqyj9rzdg6el5f5nagdfesv88sez0tc9".to_string()
        );
    }

    #[test]
    fn dispute_address() {
        let npub_1 = NostrPublicKey::from_str(KEY_A).unwrap();
        let npub_2 = NostrPublicKey::from_str(KEY_B).unwrap();
        let npub_arb = NostrPublicKey::from_str(KEY_C).unwrap();
        let timelock_duration = 100;
        let network = Network::Testnet;

        let address = escrow_address(
            &npub_1,
            &npub_2,
            Some(&npub_arb),
            Some(timelock_duration),
            network,
        )
        .unwrap();
        assert_eq!(address.address_type().unwrap(), AddressType::P2tr);
        assert_eq!(
            address.to_string(),
            "tb1paxkfvp7rra9707t8l2mk5mwuljrq6dgs0w6yey56q3d5gynp7u7s838an7".to_string()
        );
    }
}
