use bitcoin::{Address, Network, PrivateKey, XOnlyPublicKey};
use nostr::key::{PublicKey as NostrPublicKey, SecretKey as NostrSecretKey};
use secp256k1::{SECP256K1, SecretKey as SecpSecretKey};

use crate::error::Error;

/// Number of Bitcoin blocks per day assuming 10-minute intervals.
const BLOCKS_PER_DAY: u32 = 6 * 24;

/// Number of Bitcoin blocks per hour assuming 10-minute intervals.
const BLOCKS_PER_HOUR: u32 = 6;

/// Converts `days` to blocks assuming that blocks comes in 10-minute intervals.
pub fn days_to_blocks(days: u32) -> u32 {
    days * BLOCKS_PER_DAY
}

/// Converts `hours` to blocks assuming that blocks comes in 10-minute intervals.
pub fn hours_to_blocks(hours: u32) -> u32 {
    hours * BLOCKS_PER_HOUR
}

/// Converts `days` and `hours` to blocks assuming that blocks comes in 10-minute intervals.
pub fn days_hours_to_blocks(days: u32, hours: u32) -> u32 {
    days_to_blocks(days) + hours_to_blocks(hours)
}

/// Parses a network string into a [`Network`].
pub fn parse_network(network: String) -> Network {
    match network.as_str() {
        "Mainnet" => Network::Bitcoin,
        "Testnet" => Network::Testnet,
        "Signet" => Network::Signet,
        _ => panic!("Invalid network"),
    }
}

/// Parses a [`NostrPublicKey`] from a string.
pub fn parse_npub(input: &str) -> Result<NostrPublicKey, Error> {
    Ok(NostrPublicKey::parse(input)?)
}

/// Parses a [`NostrSecretKey`] from a string.
pub fn parse_nsec(input: &str) -> Result<NostrSecretKey, Error> {
    Ok(NostrSecretKey::parse(input)?)
}

/// Parses a [`NostrSecretKey`] into a [`PrivateKey`]
pub fn nsec_to_private_key(nsec: &NostrSecretKey, network: Network) -> Result<PrivateKey, Error> {
    let secret_bytes = nsec.secret_bytes();
    let secret_key = SecpSecretKey::from_slice(&secret_bytes)?;
    Ok(PrivateKey::new(secret_key, network))
}

/// Parses a [`NostrPublicKey`] to an [`XOnlyPublicKey`].
pub fn npub_to_x_only_public_key(npub: &NostrPublicKey) -> Result<XOnlyPublicKey, Error> {
    let public_bytes = npub.to_bytes();
    let x_only_pk = XOnlyPublicKey::from_slice(&public_bytes)?;
    Ok(x_only_pk)
}

/// Parses a [`NostrPublicKey`] to an [`XOnlyPublicKey`].
pub fn nsec_to_x_only_public_key(nsec: &NostrSecretKey) -> Result<XOnlyPublicKey, Error> {
    let (x_only_pk, _) = nsec.x_only_public_key(SECP256K1);
    Ok(x_only_pk)
}

/// Parses a [`NostrPublicKey`] to a P2TR [`Address`] key path spend, given a [`Network`].
pub fn npub_to_address(npub: &NostrPublicKey, network: Network) -> Result<Address, Error> {
    let public_bytes = npub.to_bytes();
    let x_only_pk = XOnlyPublicKey::from_slice(&public_bytes)?;
    let address = Address::p2tr(SECP256K1, x_only_pk, None, network);
    Ok(address)
}

#[cfg(test)]
mod tests {
    use bitcoin::hex::DisplayHex;

    use super::*;

    #[test]
    fn valid_parse_npub() {
        let npub = "npub1tv7hxxwtw4gcz4n6fpduads7lsmynh5pjedgfhvdctnulrz9rsksjx28xe";
        let npub = parse_npub(npub).unwrap();
        let pk = npub_to_x_only_public_key(&npub).unwrap();
        let expected = "5b3d7319cb755181567a485bceb61efc3649de81965a84dd8dc2e7cf8c451c2d";
        assert_eq!(pk.to_string(), expected);
    }

    #[test]
    fn valid_parse_nsec() {
        let nsec = "nsec1ezmlpxvhhjnqt9wf60tmshkye7xlwsf37dl0qlmrjuxeq7p3zahs2tukgx";
        let nsec = parse_nsec(nsec).unwrap();
        let sk = nsec_to_private_key(&nsec, Network::Bitcoin).unwrap();
        let expected = "c8b7f09997bca60595c9d3d7b85ec4cf8df74131f37ef07f63970d907831176f";
        assert_eq!(sk.to_bytes().as_hex().to_string(), expected);
    }

    #[test]
    fn odd_nsec() {
        // This motherfucker is an "odd" nsec
        let nsec = "nsec103m6x7a369k95rhtdn5w5mxsdpgyqprnysdtvhe6m0ef5xuz9d6s6emzda";
        let nsec = parse_nsec(nsec).unwrap();
        let pk = nsec_to_x_only_public_key(&nsec).unwrap();
        let expected = "2d7b3d8028c474251676708ec41f12100685b200ccbb394e5e782d73b233a8eb";
        assert_eq!(pk.to_string(), expected);
    }

    #[test]
    fn valid_npub_to_address() {
        let npub = "npub1tv7hxxwtw4gcz4n6fpduads7lsmynh5pjedgfhvdctnulrz9rsksjx28xe";
        let npub = parse_npub(npub).unwrap();
        let address = npub_to_address(&npub, Network::Bitcoin).unwrap();
        let expected = "bc1pdx0h0xkeyhx79ethugtrutlxvcswffcwa9sx823dyn09wkexdwass7v98m";
        assert_eq!(address.to_string(), expected);
    }
}
