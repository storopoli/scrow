use std::str::FromStr;

use bitcoin::{Address, Network, PublicKey};
use miniscript::Descriptor;

/// Creates a collaborative 2-of-2 multisig P2WSH [`Address`] from 2 [`PublicKey`]s
/// given a [`Network`].
pub fn new_collaborative_address(public_keys: [PublicKey; 2], network: Network) -> Address {
    let descriptor = Descriptor::<PublicKey>::from_str(&format!(
        "wsh(and_v(v:pk({}),pk({})))",
        public_keys[0], public_keys[1]
    ))
    .unwrap();
    descriptor.address(network).unwrap()
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
    let mut sorted_keys = [public_keys.to_vec(), vec![arbitrator]].concat();
    sorted_keys.sort();
    let descriptor = Descriptor::<PublicKey>::from_str(&format!(
        "wsh(andor(pk({}),pk({}),and_v(v:multi(2,{},{},{}),older({}))))",
        public_keys[0],
        public_keys[1],
        sorted_keys[0],
        sorted_keys[1],
        sorted_keys[2],
        timelock_duration,
    ))
    .unwrap();
    descriptor.address(network).unwrap()
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
    fn collaborative_address_works() {
        let public_key1 = PublicKey::from_str(KEY_A).unwrap();
        let public_key2 = PublicKey::from_str(KEY_B).unwrap();
        let network = Network::Testnet;

        let address = new_collaborative_address([public_key1, public_key2], network);

        assert_eq!(address.address_type().unwrap(), AddressType::P2wsh);
        assert_eq!(
            address.to_string(),
            "tb1qnvcs444fr6tr56zy9tjqz6mw2whgs9f76gpt2hkw3fjwh4twlzhsc3lukl".to_string()
        );
    }

    #[test]
    fn dispute_address_works() {
        let public_key1 = PublicKey::from_str(KEY_A).unwrap();
        let public_key2 = PublicKey::from_str(KEY_B).unwrap();
        let arbitrator = PublicKey::from_str(KEY_C).unwrap();
        let timelock_duration = 100;
        let network = Network::Testnet;

        let address = new_dispute_address(
            [public_key1, public_key2],
            arbitrator,
            timelock_duration,
            network,
        );
        assert_eq!(address.address_type().unwrap(), AddressType::P2wsh);
        assert_eq!(
            address.to_string(),
            "tb1quxy00fprucxfplwcmeajcynl3vfsl0qpzd8eqfpzr3m8s8frd84qf5v0ea".to_string()
        );
    }
}
