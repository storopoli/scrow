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
/* TODO
#[wasm_bindgen]
pub async fn push_transaction(host: &str, endpoint: &str, tx_hex: &str) -> Result<JsValue, JsError> {}
*/
#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use super::*;

    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    const HOST: &str = "https://mempool.space";
    const ENDPOINT: &str = "/api/v1/fees/recommended";

    // to run WASM tests: `wasm-pack test --{chrome, firefox, node, safari}`

    #[wasm_bindgen_test]
    async fn test_fetch_fees_wasm() {
        let fees = fetch_fees(HOST, ENDPOINT)
            .await
            .expect("Failed to fetch fees");

        web_sys::console::log_1(&fees);

        // Convert JsValue back to HashMap for testing
        let fees: HashMap<String, u32> =
            serde_wasm_bindgen::from_value(fees).expect("Failed to convert fees from JsValue");

        assert!(fees.contains_key("fastestFee"));
        assert!(fees.contains_key("halfHourFee"));
        assert!(fees.contains_key("hourFee"));
        assert!(fees.contains_key("economyFee"));
        assert!(fees.contains_key("minimumFee"));
    }
}
