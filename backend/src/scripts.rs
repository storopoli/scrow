use std::str::FromStr;

use bitcoin::{opcodes::all::*, Address, Network, PublicKey, ScriptBuf};
use miniscript::Descriptor;

/// Our homebrewed `OP_2` opcode.
pub const OP_2: u8 = 0x52;

/// Our homebrewed `OP_3` opcode.
pub const OP_3: u8 = 0x53;

/// Gives the key index given a list of [`PublicKey`]s and a [`PublicKey`] to find.
///
/// Under the hood it sorts the key according to `sorted_multi` and then finds the index.
pub fn find_key_index(public_keys: &[PublicKey], key: &PublicKey) -> usize {
    assert!(public_keys.contains(key));
    let mut sorted_keys = public_keys.to_vec();
    sorted_keys.sort();
    sorted_keys.iter().position(|k| k == key).unwrap()
}

/// Creates a collaborative 2-of-2 multisig P2WSH [`Address`] from 2 [`PublicKey`]s
/// given a [`Network`].
pub fn new_collaborative_address(public_keys: [PublicKey; 2], network: Network) -> Address {
    let mut sorted_keys = public_keys.to_vec();
    sorted_keys.sort();

    let mut script = ScriptBuf::new();
    script.push_opcode(OP_2.into());
    script.push_slice(sorted_keys[0].inner.serialize());
    script.push_slice(sorted_keys[1].inner.serialize());
    script.push_opcode(OP_2.into());
    script.push_opcode(OP_CHECKMULTISIG);

    Address::p2wsh(&script, network)
}

/// Creates a collaborative 2-of-2 multisig P2WSH locking script ([`ScriptBuf`]) from 2 [`PublicKey`]s.
pub fn new_collaborative_unlocking_script(public_keys: [PublicKey; 2]) -> ScriptBuf {
    let mut sorted_keys = public_keys.to_vec();
    sorted_keys.sort();

    let mut script = ScriptBuf::new();
    script.push_opcode(OP_2.into());
    script.push_slice(sorted_keys[0].inner.serialize());
    script.push_slice(sorted_keys[1].inner.serialize());
    script.push_opcode(OP_2.into());
    script.push_opcode(OP_CHECKMULTISIG);

    script
}

/// Creates a dispute-resolution 2-of-3 multisig P2WSH [`Address`] from 2 [`PublicKey`]s
/// an arbitrator [`PublicKey`] and a timelock duration in blocks
/// given a [`Network`].
///
/// The policy is as follows. Either:
///
/// - 2-of-2 multisig between the two parties without timelocks.
/// - 2-of-3 multisig between the one of the parties and the arbitrator with a timelock.
pub fn new_dispute_address(
    public_keys: [PublicKey; 2],
    arbitrator: PublicKey,
    timelock_duration: u32,
    network: Network,
) -> Address {
    let mut sorted_keys = public_keys.to_vec();
    sorted_keys.sort();
    let mut sorted_keys_all = [public_keys.to_vec(), vec![arbitrator]].concat();
    sorted_keys_all.sort();
    let descriptor = Descriptor::<PublicKey>::from_str(&format!(
        "wsh(andor(pk({}),pk({}),and_v(v:multi(2,{},{},{}),older({}))))",
        sorted_keys[0],
        sorted_keys[1],
        sorted_keys_all[0],
        sorted_keys_all[1],
        sorted_keys_all[2],
        timelock_duration,
    ))
    .unwrap();
    descriptor.address(network).unwrap()
}

/// Creates a dispute-resolution 2-of-3 multisig P2WSH locking script ([`ScriptBuf`]) from 2 [`PublicKey`]s
/// an arbitrator [`PublicKey`] and a timelock duration in blocks.
///
/// The policy is as follows. Either:
///
/// - 2-of-2 multisig between the two parties without timelocks.
/// - 2-of-3 multisig between the one of the parties and the arbitrator with a timelock.
pub fn new_dispute_unlocking_script(
    public_keys: [PublicKey; 2],
    arbitrator: PublicKey,
    timelock_duration: u32,
) -> ScriptBuf {
    let mut sorted_keys = public_keys.to_vec();
    sorted_keys.sort();
    let mut sorted_keys_all = [public_keys.to_vec(), vec![arbitrator]].concat();
    sorted_keys_all.sort();
    let descriptor = Descriptor::<PublicKey>::from_str(&format!(
        "wsh(andor(pk({}),pk({}),and_v(v:multi(2,{},{},{}),older({}))))",
        sorted_keys[0],
        sorted_keys[1],
        sorted_keys_all[0],
        sorted_keys_all[1],
        sorted_keys_all[2],
        timelock_duration,
    ))
    .unwrap();
    descriptor.explicit_script().unwrap()
}

#[cfg(test)]
mod tests {
    use bitcoin::AddressType;

    use super::*;

    // Taken from https://docs.rs/bitcoin/latest/bitcoin/struct.PublicKey.html
    const KEY_A: &str = "038f47dcd43ba6d97fc9ed2e3bba09b175a45fac55f0683e8cf771e8ced4572354";
    const KEY_B: &str = "028bde91b10013e08949a318018fedbd896534a549a278e220169ee2a36517c7aa";
    const KEY_C: &str = "032b8324c93575034047a52e9bca05a46d8347046b91a032eff07d5de8d3f2730b";

    #[test]
    fn find_key_index_works() {
        let public_keys = vec![KEY_A, KEY_B, KEY_C]
            .iter()
            .map(|key| PublicKey::from_str(key).unwrap())
            .collect::<Vec<_>>();
        let _sorted_keys = vec![KEY_B, KEY_C, KEY_A]
            .iter()
            .map(|key| PublicKey::from_str(key).unwrap())
            .collect::<Vec<_>>();
        let key_a = PublicKey::from_str(KEY_A).unwrap();
        let index = find_key_index(&public_keys, &key_a);
        assert_eq!(index, 2);
    }

    #[test]
    fn collaborative_address_works() {
        let public_key1 = PublicKey::from_str(KEY_A).unwrap();
        let public_key2 = PublicKey::from_str(KEY_B).unwrap();
        let network = Network::Testnet;

        let address_1 = new_collaborative_address([public_key1, public_key2], network);
        let address_2 = new_collaborative_address([public_key2, public_key1], network);

        assert_eq!(address_1, address_2);
        assert_eq!(address_2.address_type().unwrap(), AddressType::P2wsh);
        assert_eq!(
            address_1.to_string(),
            "tb1q256vxujwapp655r3cdk30aq3unxacln2hmq2qtfyyd92ntu6yeasfknjse".to_string()
        );
    }

    #[test]
    fn collaborative_unlocking_script_works() {
        let public_key1 = PublicKey::from_str(KEY_A).unwrap();
        let public_key2 = PublicKey::from_str(KEY_B).unwrap();

        let unlocking_script_1 = new_collaborative_unlocking_script([public_key1, public_key2]);
        let unlocking_script_2 = new_collaborative_unlocking_script([public_key1, public_key2]);
        assert_eq!(unlocking_script_1, unlocking_script_2);
        assert_eq!(
            unlocking_script_1.to_asm_string(),
            "OP_PUSHBYTES_33 028bde91b10013e08949a318018fedbd896534a549a278e220169ee2a36517c7aa OP_CHECKSIGVERIFY OP_PUSHBYTES_33 038f47dcd43ba6d97fc9ed2e3bba09b175a45fac55f0683e8cf771e8ced4572354 OP_CHECKSIG".to_string()
        );
    }

    #[test]
    fn dispute_address_works() {
        let public_key1 = PublicKey::from_str(KEY_A).unwrap();
        let public_key2 = PublicKey::from_str(KEY_B).unwrap();
        let arbitrator = PublicKey::from_str(KEY_C).unwrap();
        let timelock_duration = 100;
        let network = Network::Testnet;

        let address_1 = new_dispute_address(
            [public_key1, public_key2],
            arbitrator,
            timelock_duration,
            network,
        );
        let address_2 = new_dispute_address(
            [public_key2, public_key1],
            arbitrator,
            timelock_duration,
            network,
        );
        assert_eq!(address_1, address_2);
        assert_eq!(address_1.address_type().unwrap(), AddressType::P2wsh);
        assert_eq!(
            address_1.to_string(),
            "tb1q82d8pajf352tdskcrxzum7vwe4pypt0lfta0utaznntzpyldeh9sk2tsmd".to_string()
        );
    }

    #[test]
    fn dispute_unlocking_script_works() {
        let public_key1 = PublicKey::from_str(KEY_A).unwrap();
        let public_key2 = PublicKey::from_str(KEY_B).unwrap();
        let arbitrator = PublicKey::from_str(KEY_C).unwrap();
        let timelock_duration = 100;

        let unlocking_script_1 =
            new_dispute_unlocking_script([public_key1, public_key2], arbitrator, timelock_duration);
        let unlocking_script_2 =
            new_dispute_unlocking_script([public_key2, public_key1], arbitrator, timelock_duration);
        assert_eq!(unlocking_script_1, unlocking_script_2);
        assert_eq!(
            unlocking_script_1.to_asm_string(),
            "OP_PUSHBYTES_33 028bde91b10013e08949a318018fedbd896534a549a278e220169ee2a36517c7aa OP_CHECKSIG OP_NOTIF OP_PUSHNUM_2 OP_PUSHBYTES_33 028bde91b10013e08949a318018fedbd896534a549a278e220169ee2a36517c7aa OP_PUSHBYTES_33 032b8324c93575034047a52e9bca05a46d8347046b91a032eff07d5de8d3f2730b OP_PUSHBYTES_33 038f47dcd43ba6d97fc9ed2e3bba09b175a45fac55f0683e8cf771e8ced4572354 OP_PUSHNUM_3 OP_CHECKMULTISIGVERIFY OP_PUSHBYTES_1 64 OP_CSV OP_ELSE OP_PUSHBYTES_33 038f47dcd43ba6d97fc9ed2e3bba09b175a45fac55f0683e8cf771e8ced4572354 OP_CHECKSIG OP_ENDIF".to_string()
        );
    }
}
