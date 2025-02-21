use bitcoin::secp256k1::PublicKey as SecpPublicKey;
use bitcoin::Network;
use bitcoin::{
    bech32, hex, key::Parity, secp256k1::SecretKey as SecpSecretKey, PrivateKey, PublicKey,
    XOnlyPublicKey,
};
const PREFIX_BECH32_PUBLIC_KEY: &str = "npub";
const PREFIX_BECH32_SECRET_KEY: &str = "nsec";
const HRP_PUBLIC_KEY: bech32::Hrp = bech32::Hrp::parse_unchecked(PREFIX_BECH32_PUBLIC_KEY);
const HRP_SECRET_KEY: bech32::Hrp = bech32::Hrp::parse_unchecked(PREFIX_BECH32_SECRET_KEY);

pub fn convert_days_to_blocks(days: usize) -> usize {
    days * 144
}

pub fn convert_hours_to_blocks(hours: usize) -> usize {
    hours * 6
}

pub fn convert_days_hours_to_blocks(days: usize, hours: usize) -> usize {
    convert_days_to_blocks(days) + convert_hours_to_blocks(hours)
}

/// Checks `npub` from a bech32-encoded string
pub fn check_npub(input: String) -> bool {
    let (hrp, data) = bech32::decode(&input).expect("Not a valid npub");

    if hrp != HRP_PUBLIC_KEY || data.len() != 32 {
        return false;
    }
    true
}

/// Converts a `nsec` string to a [`SecretKey`]
pub fn convert_nsec_to_secret_key(nsec: String, network: Network) -> PrivateKey {
    let (hrp, data) = bech32::decode(&nsec).expect("Invalid bech32 string");
    if hrp != HRP_SECRET_KEY {
        panic!("Wrong prefix for nsec");
    }
    let secret_key = SecpSecretKey::from_slice(&data).expect("Invalid secret key data");
    PrivateKey::new(secret_key, network)
}

/// Convert a `nsec` bech32-encoded string to a hex-encoded string
pub fn convert_nsec_to_hex(nsec: String, network: Network) -> String {
    let secret_key: PrivateKey = convert_nsec_to_secret_key(nsec, network);
    hex::BytesToHexIter::new(secret_key.to_bytes().iter().copied()).collect()
}

/// Convert a `npub` to a [`PublicKey`]
pub fn convert_npub_to_public_key(npub: String) -> PublicKey {
    let (hrp, data) = bech32::decode(&npub).expect("Invalid bech32 string");
    if hrp != HRP_PUBLIC_KEY {
        panic!("Wrong prefix for npub");
    }
    let x_only_pk = XOnlyPublicKey::from_slice(&data).expect("Invalid public key data");
    let pk = SecpPublicKey::from_x_only_public_key(x_only_pk, Parity::Even);
    PublicKey::from(pk)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_days_to_blocks() {
        assert_eq!(convert_days_to_blocks(1), 144);
        assert_eq!(convert_days_to_blocks(2), 288);
        assert_eq!(convert_days_to_blocks(3), 432);
    }

    #[test]
    fn test_check_npub() {
        let npub = "npub1tv7hxxwtw4gcz4n6fpduads7lsmynh5pjedgfhvdctnulrz9rsksjx28xe";
        assert!(check_npub(npub.to_string()));
    }

    #[test]
    fn test_nsec_to_hex() {
        let nsec = "nsec1ezmlpxvhhjnqt9wf60tmshkye7xlwsf37dl0qlmrjuxeq7p3zahs2tukgx";
        let expected_hex = "c8b7f09997bca60595c9d3d7b85ec4cf8df74131f37ef07f63970d907831176f";

        let hex = convert_nsec_to_hex(nsec.to_string(), Network::Bitcoin);
        assert_eq!(expected_hex, hex);
    }

    #[test]
    fn test_convert_nsec_to_secret_key() {
        let nsec = "nsec1vl029mgpspedva04g90vltkh6fvh240zqtv9k0t9af8935ke9laqsnlfe5";
        let secret_key = convert_nsec_to_secret_key(nsec.to_string(), Network::Bitcoin);
        let expected_hex = "67dea2ed018072d675f5415ecfaed7d2597555e202d85b3d65ea4e58d2d92ffa";
        let hex: String = hex::BytesToHexIter::new(secret_key.to_bytes().iter().copied()).collect();
        assert_eq!(expected_hex, hex);
    }

    #[test]
    fn test_convert_npub_to_public_key() {
        let npub = "npub10elfcs4fr0l0r8af98jlmgdh9c8tcxjvz9qkw038js35mp4dma8qzvjptg";
        let public_key: PublicKey = convert_npub_to_public_key(npub.to_string());
        // NOTE: adding 02 for the even parity
        let expected_hex = "027e7e9c42a91bfef19fa929e5fda1b72e0ebc1a4c1141673e2794234d86addf4e";
        let hex: String = hex::BytesToHexIter::new(public_key.to_bytes().iter().copied()).collect();
        assert_eq!(expected_hex, hex);
    }
}
