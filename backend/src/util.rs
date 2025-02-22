use bitcoin::key::Secp256k1;
use bitcoin::secp256k1::PublicKey as SecpPublicKey;
use bitcoin::Network;
use bitcoin::{
    bech32, hex, key::Parity, secp256k1::SecretKey as SecpSecretKey, PrivateKey, PublicKey,
    XOnlyPublicKey,
};
use miniscript::ToPublicKey;
use wasm_bindgen::prelude::*;

const PREFIX_BECH32_PUBLIC_KEY: &str = "npub";
const PREFIX_BECH32_SECRET_KEY: &str = "nsec";
const HRP_PUBLIC_KEY: bech32::Hrp = bech32::Hrp::parse_unchecked(PREFIX_BECH32_PUBLIC_KEY);
const HRP_SECRET_KEY: bech32::Hrp = bech32::Hrp::parse_unchecked(PREFIX_BECH32_SECRET_KEY);

#[wasm_bindgen]
pub fn convert_days_to_blocks_wasm(days: usize) -> usize {
    days * 144
}

#[wasm_bindgen]
pub fn convert_hours_to_blocks_wasm(hours: usize) -> usize {
    hours * 6
}

#[wasm_bindgen]
pub fn convert_days_hours_to_blocks_wasm(days: usize, hours: usize) -> usize {
    convert_days_to_blocks_wasm(days) + convert_hours_to_blocks_wasm(hours)
}

#[wasm_bindgen]
/// Checks `npub` from a bech32-encoded string
pub fn check_npub_wasm(input: String) -> bool {
    let (hrp, data) = bech32::decode(&input).expect("Not a valid npub");

    if hrp != HRP_PUBLIC_KEY || data.len() != 32 {
        return false;
    }
    true
}

/// Converts a `nsec` string to a [`SecretKey`]
fn convert_nsec_to_secret_key(nsec: String, network: Network) -> PrivateKey {
    let (hrp, data) = bech32::decode(&nsec).expect("Invalid bech32 string");
    if hrp != HRP_SECRET_KEY {
        panic!("Wrong prefix for nsec");
    }
    let secret_key = SecpSecretKey::from_slice(&data).expect("Invalid secret key data");
    PrivateKey::new(secret_key, network)
}

/// Convert a `nsec` bech32-encoded string to a hex-encoded string
fn convert_nsec_to_hex(nsec: String, network: Network) -> String {
    let secret_key: PrivateKey = convert_nsec_to_secret_key(nsec, network);
    hex::BytesToHexIter::new(secret_key.to_bytes().iter().copied()).collect()
}

#[wasm_bindgen]
/// Convert a `nsec` bech32-encoded string to a hex-encoded string to wasm
pub fn convert_nsec_to_hex_wasm(nsec: String, network: String) -> String {
    let network = match network.as_str() {
        "Mainnet" => Network::Bitcoin,
        "Testnet" => Network::Testnet,
        "Signet" => Network::Signet,
        "Mutinynet" => Network::Signet,
        _ => panic!("Invalid network"),
    };
    convert_nsec_to_hex(nsec, network)
}

/// Convert a network to a typed [`Network`]
pub fn convert_network_to_typed(network: String) -> Network {
    match network.as_str() {
        "Mainnet" => Network::Bitcoin,
        "Testnet" => Network::Testnet,
        "Signet" => Network::Signet,
        "Mutinynet" => Network::Signet,
        _ => panic!("Invalid network"),
    }
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

/// Calculates a pub_key from a nsec string
fn pub_key_derivation_to_sign(nsec: String, network: Network) -> SecpPublicKey {
    let sec_key = convert_nsec_to_secret_key(nsec, network);
    let pub_key: SecpPublicKey = SecpPublicKey::from_secret_key(&Secp256k1::new(), &sec_key.inner);
    let x_only_pk = pub_key.to_x_only_pubkey();
    SecpPublicKey::from_x_only_public_key(x_only_pk, Parity::Even)
}

#[wasm_bindgen]
/// Calculates a pub_key from a nsec string to wasm
/// Returns a hex-encoded string
pub fn pub_key_derivation_to_sign_wasm(nsec: String, network: String) -> String {
    let network = match network.as_str() {
        "Mainnet" => Network::Bitcoin,
        "Testnet" => Network::Testnet,
        "Signet" => Network::Signet,
        "Mutinynet" => Network::Signet,
        _ => panic!("Invalid network"),
    };
    let public_key = pub_key_derivation_to_sign(nsec, network);
    hex::BytesToHexIter::new(public_key.serialize().iter().copied()).collect()
}

#[wasm_bindgen]
/// Convert a `npub` bech32-encoded string to a hex-encoded string
pub fn convert_npub_to_hex_wasm(npub: String) -> String {
    let public_key: PublicKey = convert_npub_to_public_key(npub);
    hex::BytesToHexIter::new(public_key.to_bytes().iter().copied()).collect()
}

#[cfg(test)]
mod tests {
    use bitcoin::key::Secp256k1;
    use miniscript::ToPublicKey;

    use super::*;

    #[test]
    fn test_convert_days_to_blocks() {
        assert_eq!(convert_days_to_blocks_wasm(1), 144);
        assert_eq!(convert_days_to_blocks_wasm(2), 288);
        assert_eq!(convert_days_to_blocks_wasm(3), 432);
    }

    #[test]
    fn test_check_npub() {
        let npub = "npub1tv7hxxwtw4gcz4n6fpduads7lsmynh5pjedgfhvdctnulrz9rsksjx28xe";
        assert!(check_npub_wasm(npub.to_string()));
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

    #[test]
    fn test_convert_npub_to_hex() {
        let npub = "npub10elfcs4fr0l0r8af98jlmgdh9c8tcxjvz9qkw038js35mp4dma8qzvjptg";
        let expected_hex = "027e7e9c42a91bfef19fa929e5fda1b72e0ebc1a4c1141673e2794234d86addf4e";
        let hex = convert_npub_to_hex_wasm(npub.to_string());
        assert_eq!(expected_hex, hex);
    }

    #[test]
    fn test_convert_nsec_to_hex_wasm() {
        let nsec = "nsec1vl029mgpspedva04g90vltkh6fvh240zqtv9k0t9af8935ke9laqsnlfe5";
        let expected_hex = "67dea2ed018072d675f5415ecfaed7d2597555e202d85b3d65ea4e58d2d92ffa";
        let hex = convert_nsec_to_hex_wasm(nsec.to_string(), "Mainnet".to_string());
        assert_eq!(expected_hex, hex);
    }

    #[test]
    fn test_convert_nsec_to_hex() {
        let nsec = "nsec103m6x7a369k95rhtdn5w5mxsdpgyqprnysdtvhe6m0ef5xuz9d6s6emzda";
        let expected_hex = "7c77a37bb1d16c5a0eeb6ce8ea6cd06850400473241ab65f3adbf29a1b822b75";
        let hex = convert_nsec_to_hex_wasm(nsec.to_string(), "Mainnet".to_string());
        assert_eq!(expected_hex, hex);
    }

    #[test]
    fn test_pub_key_derivation_to_sign() {
        let nsec = "nsec103m6x7a369k95rhtdn5w5mxsdpgyqprnysdtvhe6m0ef5xuz9d6s6emzda";
        let sec_key = convert_nsec_to_secret_key(nsec.to_string(), Network::Bitcoin);
        let pub_key: SecpPublicKey =
            SecpPublicKey::from_secret_key(&Secp256k1::new(), &sec_key.inner);
        let pub_key = pub_key.to_x_only_pubkey();
        let pub_key_hex: String =
            hex::BytesToHexIter::new(pub_key.serialize().iter().copied()).collect();
        let expected_hex = "2d7b3d8028c474251676708ec41f12100685b200ccbb394e5e782d73b233a8eb";
        assert_eq!(expected_hex, pub_key_hex);
    }

    #[test]
    fn test_pub_key_derivation_to_sign_ypar() {
        let nsec = "nsec103m6x7a369k95rhtdn5w5mxsdpgyqprnysdtvhe6m0ef5xuz9d6s6emzda";
        let public_key = pub_key_derivation_to_sign(nsec.to_string(), Network::Bitcoin);
        let pub_key_hex: String =
            hex::BytesToHexIter::new(public_key.serialize().iter().copied()).collect();
        let expected_hex = "022d7b3d8028c474251676708ec41f12100685b200ccbb394e5e782d73b233a8eb";
        assert_eq!(expected_hex, pub_key_hex);
    }
}
