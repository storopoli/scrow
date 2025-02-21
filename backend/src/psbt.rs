use std::collections::BTreeMap;

use bitcoin::{PrivateKey, Psbt, PublicKey};
use secp256k1::SECP256K1;

/// Signs a [`Psbt`] with a given [`PrivateKey`].
pub fn sign_psbt(mut psbt: Psbt, private_key: PrivateKey) -> Psbt {
    // Get the BTreeMap<PublicKey, PrivateKey> for signing.
    let public_key = private_key.public_key(SECP256K1);
    let mut map: BTreeMap<PublicKey, PrivateKey> = BTreeMap::new();
    map.insert(public_key, private_key);
    psbt.sign(&map, SECP256K1).unwrap();
    psbt
}
