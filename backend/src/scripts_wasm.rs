use wasm_bindgen::prelude::*;

use crate::{
    scripts::{new_collaborative_unlocking_script, new_dispute_unlocking_script},
    util,
};
/// Creates a collaborative 2-of-2 multisig P2WSH locking script ([`ScriptBuf`]) from 2 [`PublicKey`]s.
#[wasm_bindgen]
pub fn new_collaborative_unlocking_script_wasm(npub: Vec<String>) -> String {
    let public_key_1 = util::npub_to_public_key(npub[0].clone());
    let public_key_2 = util::npub_to_public_key(npub[1].clone());
    let public_keys = [public_key_1, public_key_2];
    let script = new_collaborative_unlocking_script(public_keys);
    script.to_hex_string()
}

/// Creates a dispute-resolution 2-of-3 multisig P2WSH locking script ([`ScriptBuf`]) from 2 [`PublicKey`]s
/// an arbitrator [`PublicKey`] and a timelock duration in blocks.
///
/// The policy is as follows. Either:
///
/// - 2-of-2 multisig between the two parties without timelocks.
/// - 2-of-3 multisig between the one of the parties and the arbitrator with a timelock.
#[wasm_bindgen]
pub fn new_dispute_unlocking_script_wasm(
    npub: Vec<String>,
    npub_arbitrator: String,
    timelock_duration: u32,
) -> String {
    let public_key_1 = util::npub_to_public_key(npub[0].clone());
    let public_key_2 = util::npub_to_public_key(npub[1].clone());
    let arbitrator = util::npub_to_public_key(npub_arbitrator);
    let public_keys = [public_key_1, public_key_2];
    let script = new_dispute_unlocking_script(public_keys, arbitrator, timelock_duration);
    script.to_hex_string()
}
