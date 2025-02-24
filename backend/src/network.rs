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
    #![allow(deprecated)]

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
    use secp256k1::{Message, Parity, SecretKey, SECP256K1};
    use std::collections::BTreeMap;

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
}
