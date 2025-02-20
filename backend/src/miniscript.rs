use std::str::FromStr;

use bitcoin::{Address, Network, PublicKey};
use miniscript::Descriptor;

/// Creates a collaborative 2-of-2 multisig P2WSH [`Address`] from 2 [`PublicKey`]s
/// given a [`Network`].
pub fn new_collaborative_address(public_keys: [PublicKey; 2], network: Network) -> Address {
    let descriptor = Descriptor::<PublicKey>::from_str(&format!(
        "wsh(or_b(pk({}),s:pk({})))",
        public_keys[0].to_string(),
        public_keys[1].to_string()
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

    #[test]
    fn collaborative_address_works() {
        let public_key1 = PublicKey::from_str(KEY_A).unwrap();
        let public_key2 = PublicKey::from_str(KEY_B).unwrap();
        let network = Network::Testnet;

        let address = new_collaborative_address([public_key1, public_key2], network);
        assert_eq!(address.address_type().unwrap(), AddressType::P2wsh);
        assert_eq!(
            address.to_string(),
            "tb1qjw077s78d8uqswdsfrffex4926488npk88dkfanqd8ynx5c06urq7umjsq".to_string()
        );
    }
}
