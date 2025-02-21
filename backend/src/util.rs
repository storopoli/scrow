use bitcoin::{bech32, hex, secp256k1::SecretKey};

const PREFIX_BECH32_PUBLIC_KEY: &str = "npub";
const PREFIX_BECH32_SECRET_KEY: &str = "nsec";
const HRP_PUBLIC_KEY: bech32::Hrp = bech32::Hrp::parse_unchecked(PREFIX_BECH32_PUBLIC_KEY);
const HRP_SECRET_KEY: bech32::Hrp = bech32::Hrp::parse_unchecked(PREFIX_BECH32_SECRET_KEY);

pub fn convert_days_to_blocks(days: usize) -> usize {
    days * 144
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
pub fn convert_nsec_to_secret_key(nsec: String) -> SecretKey {
    let (hrp, data) = bech32::decode(&nsec).expect("Invalid bech32 string");
    if hrp != HRP_SECRET_KEY {
        panic!("Wrong prefix for nsec");
    }
    SecretKey::from_slice(&data).expect("Invalid secret key data")
}

/// Convert a `nsec` bech32-encoded string to a hex-encoded string
pub fn convert_nsec_to_hex(nsec: String) -> String {
    let secret_key: SecretKey = convert_nsec_to_secret_key(nsec);
    let hex: String = hex::BytesToHexIter::new(secret_key.secret_bytes().iter().copied()).collect();
    hex
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

        let hex = convert_nsec_to_hex(nsec.to_string());
        assert_eq!(expected_hex, hex);
    }

    #[test]
    fn test_convert_nsec_to_secret_key() {
        let nsec = "nsec1ezmlpxvhhjnqt9wf60tmshkye7xlwsf37dl0qlmrjuxeq7p3zahs2tukgx";
        let secret_key = convert_nsec_to_secret_key(nsec.to_string());
        let expected_hex = "c8b7f09997bca60595c9d3d7b85ec4cf8df74131f37ef07f63970d907831176f";
        let hex: String = hex::BytesToHexIter::new(secret_key.secret_bytes().iter().copied()).collect();
        assert_eq!(expected_hex, hex);
    }
}
