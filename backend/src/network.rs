#![allow(unused)]

use reqwest::Client;
use std::collections::HashMap;
use url::Url;
use wasm_bindgen::prelude::*;

/// Fetches recommended transaction fees in sat/vB from a `mempool.space` compliant API
/// and returns a [`HashMap<String, u32>`]
///
/// If using MutinyNet, the host MUST be `https://mutinynet.com`
#[wasm_bindgen]
pub async fn fetch_fees(host: &str, endpoint: &str) -> Result<JsValue, JsError> {
    // build full URL
    let url = format!("{}{}", host, endpoint);

    // check URL for correctness
    let _parsed_url =
        Url::parse(&url).map_err(|e| JsError::new(&format!("Error parsing URL: {}", e)))?;

    let client = Client::new();

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| JsError::new(&e.to_string()))?;

    let response_text = response
        .text()
        .await
        .map_err(|e| JsError::new(&e.to_string()))?;

    let fees: HashMap<String, u32> = serde_json::from_str(&response_text)
        .map_err(|e| JsError::new(&format!("Failed to parse JSON response: {}", e)))?;

    // convert to JsValue for WASM compatibility
    Ok(serde_wasm_bindgen::to_value(&fees)
        .map_err(|e| JsError::new(&format!("Failed to convert to JS value: {}", e)))
        .unwrap())
}

/// Pushes a signed raw transaction to a `mempool.space` compliant API and returns a
/// tuple of success status and TXID
///
/// "The transaction should be provided as hex in the request body. The txid will be returned on success."
///
/// If using MutinyNet, the host MUST be `https://mutinynet.com`
#[wasm_bindgen]
pub async fn push_transaction(
    host: &str,
    endpoint: &str,
    tx_hex: &str,
) -> Result<JsValue, JsError> {
    let url = format!("{}{}", host, endpoint);

    let _parsed_url =
        Url::parse(&url).map_err(|e| JsError::new(&format!("Error parsing URL: {}", e)))?;

    let client = Client::new();

    let response = client
        .post(url)
        .header("Content-Type", "text/plain")
        .body(tx_hex.to_string())
        .send()
        .await
        .map_err(|e| JsError::new(&e.to_string()))?;

    let response_text = response
        .text()
        .await
        .map_err(|e| JsError::new(&e.to_string()))?;

    Ok(serde_wasm_bindgen::to_value(&response_text)
        .map_err(|e| JsError::new(&format!("Failed to convert to JS value: {}", e)))
        .unwrap())
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use super::*;

    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    // to run WASM tests: wasm-pack test --{chrome, firefox, node, safari}

    use bitcoin::{
        absolute::LockTime, consensus, ecdsa, hex::DisplayHex, sighash::SighashCache, transaction,
        Address, Amount, BlockHash, CompressedPublicKey, Network, OutPoint, PrivateKey, Psbt,
        PublicKey, Transaction, TxIn, TxOut, Witness,
    };
    use corepc_node::Node;
    use miniscript::ToPublicKey;
    use secp256k1::{Message, Parity, SecretKey, SECP256K1};
    use std::collections::BTreeMap;

    use crate::sign::sign_psbt;

    const HOST: &str = "https://mutinynet.com";
    const FEES_ENDPOINT: &str = "/api/v1/fees/recommended";
    const PUSHTX_ENDPOINT: &str = "/api/transaction";

    #[wasm_bindgen_test]
    async fn test_fetch_fees_wasm() {
        let fees = fetch_fees(HOST, FEES_ENDPOINT)
            .await
            .expect("Failed to fetch fees");

        // open your browser's console to see this
        web_sys::console::log_1(&fees);

        let fees: HashMap<String, u32> =
            serde_wasm_bindgen::from_value(fees).expect("Failed to convert fees from JsValue");

        assert!(fees.contains_key("fastestFee"));
        assert!(fees.contains_key("halfHourFee"));
        assert!(fees.contains_key("hourFee"));
        assert!(fees.contains_key("economyFee"));
        assert!(fees.contains_key("minimumFee"));
    }

    #[wasm_bindgen_test]
    #[ignore = "reason"]
    async fn test_push_transaction() {
        // Setup regtest node and clients.
        let bitcoind = Node::from_downloaded().unwrap();
        let btc_client = &bitcoind.client;

        // Get network.
        let network = btc_client
            .get_blockchain_info()
            .expect("must get blockchain info")
            .chain;
        let network = network.parse::<Network>().expect("network must be valid");

        const PRIVKEY_HEX: &str =
            "67dea2ed018072d675f5415ecfaed7d2597555e202d85b3d65ea4e58d2d92ffa";

        let sec_key = PRIVKEY_HEX
            .parse::<SecretKey>()
            .expect("must parse secret key");
        let private_key = PrivateKey::new(sec_key, network);
        let public_key = private_key.public_key(SECP256K1);

        // Fund a SegWit-v0 address from the PublicKey.
        // Mine until maturity (101 blocks in Regtest).
        let compressed_pk: CompressedPublicKey = public_key.try_into().unwrap();
        let funded_address = Address::p2wpkh(&compressed_pk, network);
        println!("Funded address: {}", funded_address);
        let coinbase_block = btc_client
            .generate_to_address(101, &funded_address)
            .expect("must be able to generate blocks")
            .0
            .first()
            .expect("must be able to get the blocks")
            .parse::<BlockHash>()
            .expect("must parse");
        let coinbase_txid = btc_client
            .get_block(coinbase_block)
            .expect("must be able to get coinbase block")
            .coinbase()
            .expect("must be able to get the coinbase transaction")
            .compute_txid();

        // We're sending 49.999 and 0.001 will be fees.
        let send_amount = Amount::from_btc(49.999).unwrap();
        let send_address = btc_client.new_address().unwrap();

        // Create the transaction.
        let funding_input = OutPoint {
            txid: coinbase_txid,
            vout: 0,
        };
        let inputs = vec![TxIn {
            previous_output: funding_input,
            ..Default::default()
        }];
        let outputs = vec![TxOut {
            value: send_amount,
            script_pubkey: send_address.script_pubkey(),
        }];
        let unsigned = Transaction {
            version: transaction::Version(2),
            input: inputs,
            output: outputs,
            lock_time: LockTime::ZERO,
        };
        let psbt = Psbt::from_unsigned_tx(unsigned).expect("it is unsigned");
        println!("Unsigned PSBT: {psbt:?}");

        // Sign the Psbt.
        let public_key = private_key.public_key(SECP256K1);
        let mut map: BTreeMap<PublicKey, PrivateKey> = BTreeMap::new();
        map.insert(public_key, private_key);
        // Get the Prevout.
        let coinbase_amount = Amount::from_btc(50.0).unwrap();
        let prevout = TxOut {
            value: coinbase_amount,
            script_pubkey: funded_address.script_pubkey(),
        };
        // Get the Redeem Script.
        let redeem_script = funded_address.script_pubkey();
        #[allow(deprecated)]
        let mut signed_psbt = sign_psbt(psbt, private_key, prevout, redeem_script);
        println!("Signed PSBT: {signed_psbt:?}");

        // let finalized_psbt = signed_psbt.finalize(SECP256K1).unwrap();
        let final_script_witness = {
            let (_, sig) = signed_psbt.inputs[0].partial_sigs.iter().next().unwrap();
            let pk = secp256k1::PublicKey::from_x_only_public_key(
                public_key.to_x_only_pubkey(),
                Parity::Even,
            );
            Witness::p2wpkh(sig, &pk)
        };
        // Clear all the data fields as per the spec.
        signed_psbt.inputs[0].final_script_witness = Some(final_script_witness);
        signed_psbt.inputs[0].partial_sigs = BTreeMap::new();
        signed_psbt.inputs[0].sighash_type = None;
        signed_psbt.inputs[0].redeem_script = None;
        signed_psbt.inputs[0].witness_script = None;
        signed_psbt.inputs[0].bip32_derivation = BTreeMap::new();
        println!("Finalized PSBT: {signed_psbt:?}");

        // Test if the transaction is valid.
        let signed_tx = signed_psbt.extract_tx().unwrap();
        let signed_tx_hex = consensus::serialize(&signed_tx).as_hex().to_string();

        let r = push_transaction(HOST, PUSHTX_ENDPOINT, &signed_tx_hex)
            .await
            .unwrap();

        // open your browser's console to see this
        web_sys::console::log_1(&r);
    }
}
