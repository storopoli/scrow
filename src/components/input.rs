//! Input Validation Components.

use bitcoin::{Address, Amount, FeeRate, Transaction, Txid, consensus};
use dioxus::prelude::*;

#[cfg(debug_assertions)]
use dioxus::logger::tracing::trace;
use secp256k1::schnorr;

use crate::{
    ESPLORA_ENDPOINT, NETWORK,
    util::{npub_to_address, parse_network, parse_npub, parse_nsec},
};

/// Nostr `npub` input validation component.
#[component]
pub(crate) fn NpubInput(mut update_var: Signal<String>, label: String, id: String) -> Element {
    let mut has_error = use_signal(|| false);
    let mut validate_npub = move |input: &str| {
        let result = parse_npub(input);
        *has_error.write() = result.is_err() && !input.is_empty();
        if result.is_ok() || input.is_empty() {
            update_var.set(input.to_string());
        }
    };

    let input_class = if *has_error.read() {
        "shadow-sm focus:ring-red-500 focus:border-red-500 block w-full sm:text-sm border-red-300 rounded-md p-2 border bg-red-50"
    } else {
        "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border"
    };
    rsx! {
        div { class: "sm:col-span-3",
            label {
                r#for: id.as_str(),
                class: "block text-sm font-medium text-gray-700",
                {label}
            }
            div { class: "mt-1",
                input {
                    r#type: "text",
                    name: id.as_str(),
                    id: id.as_str(),
                    class: input_class,
                    placeholder: "npub...",
                    oninput: move |event| {
                        #[cfg(debug_assertions)]
                        trace!(% id, % update_var, event_value =% event.value(), "Set npub");
                        validate_npub(&event.value());
                    },
                }
            }
            if *has_error.read() {
                p { class: "mt-2 text-xs text-red-600",
                    "Invalid npub format. Please enter a valid Nostr public key."
                }
            }
        }
    }
}

/// Nostr `npub` input validation component that also derives the address.
#[component]
pub(crate) fn NpubInputDerivedAddress(
    mut update_var: Signal<String>,
    mut update_address: Signal<String>,
    label: String,
    id: String,
    col_span: u8,
) -> Element {
    let mut has_error = use_signal(|| false);

    let mut validate_and_derive = move |input: &str| {
        let parsed_npub = parse_npub(input);
        has_error.set(parsed_npub.is_err() && !input.is_empty());

        update_var.set(input.to_string());

        if let Ok(parsed_npub) = parsed_npub {
            if let Ok(parsed_network) = parse_network(&NETWORK.read()) {
                if let Ok(address) = npub_to_address(&parsed_npub, parsed_network) {
                    let derived_address_str = address.to_string();
                    #[cfg(debug_assertions)]
                    trace!(
                        % derived_address_str, % update_address, event_value =% input,
                        "Set derived address"
                    );
                    update_address.set(derived_address_str);
                    return;
                }
            }
        }

        // Clear the address if validation fails
        if !input.is_empty() {
            update_address.set(String::new());
        }
    };

    let input_class = if *has_error.read() {
        "shadow-sm focus:ring-red-500 focus:border-red-500 block w-full sm:text-sm border-red-300 rounded-md p-2 border bg-red-50"
    } else {
        "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border"
    };

    rsx! {
        div { class: format!("sm:col-span-{col_span}").as_str(),
            label {
                r#for: id.as_str(),
                class: "block text-sm font-medium text-gray-700",
                {label}
            }
            div { class: "mt-1",
                input {
                    r#type: "text",
                    name: id.as_str(),
                    id: id.as_str(),
                    class: input_class,
                    placeholder: "npub...",
                    oninput: move |event| {
                        #[cfg(debug_assertions)]
                        trace!(% id, % update_var, event_value =% event.value(), "Set npub");
                        validate_and_derive(&event.value());
                    },
                }
            }
            if *has_error.read() {
                p { class: "mt-2 text-xs text-red-600",
                    "Invalid npub format. Please enter a valid Nostr public key."
                }
            }
        }
    }
}

/// Bitcoin BTC amount input validation component.
#[component]
pub(crate) fn BitcoinInput(mut update_var: Signal<String>, label: String, id: String) -> Element {
    let mut has_error = use_signal(|| false);

    let mut validate_amount = move |input: &str| {
        if input.is_empty() {
            *has_error.write() = false;
            update_var.set(input.to_string());
            return;
        }

        match input.parse::<f64>() {
            Ok(amount) => {
                let is_valid = Amount::from_btc(amount).is_ok();
                *has_error.write() = !is_valid;
                update_var.set(input.to_string());
            }
            Err(_) => {
                *has_error.write() = true;
            }
        }
    };

    let input_class = if *has_error.read() {
        "shadow-sm focus:ring-red-500 focus:border-red-500 block w-full sm:text-sm border-red-300 rounded-md p-2 border bg-red-50"
    } else {
        "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border"
    };

    rsx! {
        div { class: "sm:col-span-3",
            label {
                r#for: id.as_str(),
                class: "block text-sm font-medium text-gray-700",
                {label}
            }
            div { class: "mt-1",
                input {
                    r#type: "number",
                    min: "0.00000001",
                    max: "100.0",
                    step: "0.00000001",
                    name: id.as_str(),
                    id: id.as_str(),
                    class: input_class,
                    placeholder: "0.00000000",
                    oninput: move |event| {
                        #[cfg(debug_assertions)]
                        trace!(% update_var, event_value =% event.value(), "Set Bitcoin amount");
                        validate_amount(&event.value());
                    },
                }
            }
            if *has_error.read() {
                p { class: "mt-2 text-xs text-red-600",
                    "Amount must be between 0.00000001 and 100 BTC."
                }
            }
        }
    }
}

/// Fee rate input validation component.
#[component]
pub(crate) fn FeeRateInput(mut update_var: Signal<String>, label: String, id: String) -> Element {
    let mut has_error = use_signal(|| false);

    let mut validate_fee_rate = move |input: &str| {
        if input.is_empty() {
            *has_error.write() = false;
            update_var.set(input.to_string());
            return;
        }

        match input.parse::<u64>() {
            Ok(rate) => {
                let is_valid = rate > 0 && FeeRate::from_sat_per_vb(rate).is_some();
                *has_error.write() = !is_valid;
                update_var.set(input.to_string());
            }
            Err(_) => {
                *has_error.write() = true;
            }
        }
    };

    let input_class = if *has_error.read() {
        "shadow-sm focus:ring-red-500 focus:border-red-500 block w-full sm:text-sm border-red-300 rounded-md p-2 border bg-red-50"
    } else {
        "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border"
    };

    rsx! {
        div { class: "sm:col-span-3",
            label {
                r#for: id.as_str(),
                class: "block text-sm font-medium text-gray-700",
                {label}
            }
            div { class: "mt-1",
                input {
                    r#type: "number",
                    min: "1",
                    step: "1",
                    name: id.as_str(),
                    id: id.as_str(),
                    class: input_class,
                    placeholder: "1",
                    oninput: move |event| {
                        #[cfg(debug_assertions)]
                        trace!(% update_var, event_value =% event.value(), "Set fee rate");
                        validate_fee_rate(&event.value());
                    },
                }
            }
            if *has_error.read() {
                p { class: "mt-2 text-xs text-red-600", "Fee rate must be a positive integer." }
            }
        }
    }
}

/// Network input validation component.
#[component]
pub(crate) fn NetworkInput(label: String, id: String) -> Element {
    // Function to get the default endpoint for a network
    let get_default_endpoint = |network: &str| -> String {
        match network {
            "Mainnet" => "https://mempool.space/api",
            "Testnet" => "https://mempool.space/testnet/api",
            "Signet" => "https://mempool.space/signet/api",
            "Regtest" => "http://127.0.0.1:3002/api",
            _ => "https://mempool.space/api",
        }
        .to_string()
    };

    let update_esplora_endpoint = move |network: &str| {
        // Set appropriate Esplora endpoint based on selected network
        let endpoint = get_default_endpoint(network);
        *ESPLORA_ENDPOINT.write() = endpoint;
    };

    // Get the default endpoint for the current network
    let default_endpoint = get_default_endpoint(&NETWORK.read());

    rsx! {
        div { class: "sm:col-span-3",
            label {
                r#for: id.as_str(),
                class: "block text-sm font-medium text-gray-700",
                {label}
            }
            div { class: "mt-1",
                select {
                    id: "network",
                    name: "network",
                    class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                    oninput: move |event| {
                        #[cfg(debug_assertions)]
                        trace!(% NETWORK, event_value =% event.value(), "Set network");
                        let network_value = event.value();
                        *NETWORK.write() = network_value.clone();
                        update_esplora_endpoint(&network_value);
                    },
                    value: NETWORK.read().clone(),
                    option { value: "Mainnet", "Mainnet" }
                    option { value: "Testnet", "Testnet" }
                    option { value: "Signet", "Signet" }
                    option { value: "Regtest", "Regtest" }
                }
            }
            // Using two separate paragraphs
            p { class: "mt-2 text-xs text-gray-500", "Default Esplora endpoint: {default_endpoint}" }
            p { class: "text-xs text-gray-500", "Current Esplora endpoint: {ESPLORA_ENDPOINT.read()}" }
        }
    }
}

/// Esplora backend input validation component.
#[component]
pub(crate) fn EsploraInput() -> Element {
    let mut has_error = use_signal(|| false);

    let mut validate_url = move |input: &str| {
        if input.is_empty() {
            *has_error.write() = false;
            *ESPLORA_ENDPOINT.write() = input.to_string();
            return;
        }

        // Simple URL validation
        let is_valid = input.starts_with("http://") || input.starts_with("https://");
        *has_error.write() = !is_valid;
        *ESPLORA_ENDPOINT.write() = input.to_string();
    };

    let input_class = if *has_error.read() {
        "shadow-sm focus:ring-red-500 focus:border-red-500 block w-full sm:text-sm border-red-300 rounded-md p-2 border bg-red-50"
    } else {
        "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border"
    };

    rsx! {
        div { class: "sm:col-span-6",
            label {
                r#for: "esplora-url",
                class: "block text-sm font-medium text-gray-700",
                "Esplora API Backend URL"
            }
            div { class: "mt-1",
                input {
                    r#type: "url",
                    name: "esplora-url",
                    id: "esplora-url",
                    class: input_class,
                    placeholder: "https://mempool.space/api",
                    value: ESPLORA_ENDPOINT.read().clone(),
                    oninput: move |event| {
                        #[cfg(debug_assertions)]
                        trace!(% ESPLORA_ENDPOINT, event_value =% event.value(), "Set Eslora endpoint");
                        validate_url(&event.value());
                    },
                }
            }
            if *has_error.read() {
                p { class: "mt-2 text-xs text-red-600",
                    "Invalid URL format. URL should start with http:// or https://"
                }
            } else {
                p { class: "mt-2 text-xs text-gray-500",
                    "Default for mainnet: https://mempool.space/api"
                }
            }
        }
    }
}

/// Timelock input validation component.
#[component]
pub(crate) fn TimelockInput(
    mut update_day_var: Signal<String>,
    mut update_hour_var: Signal<String>,
) -> Element {
    let mut days_has_error = use_signal(|| false);
    let mut hours_has_error = use_signal(|| false);

    let mut validate_days = move |input: &str| {
        if input.is_empty() {
            *days_has_error.write() = false;
            update_day_var.set(input.to_string());
            return;
        }

        match input.parse::<u32>() {
            Ok(days) => {
                // A very large value (e.g., over 1,000 days) might be a mistake
                let is_valid = days <= 1_000;
                *days_has_error.write() = !is_valid;
                update_day_var.set(input.to_string());
            }
            Err(_) => {
                *days_has_error.write() = true;
            }
        }
    };

    let mut validate_hours = move |input: &str| {
        if input.is_empty() {
            *hours_has_error.write() = false;
            update_hour_var.set(input.to_string());
            return;
        }

        match input.parse::<u32>() {
            Ok(hours) => {
                // Hours should be 0-23
                let is_valid = hours < 24;
                *hours_has_error.write() = !is_valid;
                update_hour_var.set(input.to_string());
            }
            Err(_) => {
                *hours_has_error.write() = true;
            }
        }
    };

    let days_input_class = if *days_has_error.read() {
        "shadow-sm focus:ring-red-500 focus:border-red-500 block w-full sm:text-sm border-red-300 rounded-md p-2 border bg-red-50"
    } else {
        "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border"
    };

    let hours_input_class = if *hours_has_error.read() {
        "shadow-sm focus:ring-red-500 focus:border-red-500 block w-full sm:text-sm border-red-300 rounded-md p-2 border bg-red-50"
    } else {
        "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border"
    };

    rsx! {
        div { class: "sm:col-span-3",
            div { class: "grid grid-cols-2 gap-4",
                div {
                    label {
                        r#for: "timelock-days",
                        class: "block text-sm font-medium text-gray-700",
                        "Timelock (Days)"
                    }
                    div { class: "mt-1",
                        input {
                            r#type: "number",
                            min: "0",
                            step: "1",
                            name: "timelock-days",
                            id: "timelock-days",
                            class: days_input_class,
                            placeholder: "0",
                            oninput: move |event| {
                                #[cfg(debug_assertions)]
                                trace!(% update_day_var, event_value =% event.value(), "Set timelock days");
                                validate_days(&event.value());
                            },
                        }
                    }
                    if *days_has_error.read() {
                        p { class: "mt-2 text-xs text-red-600", "Days should be between 0 and 1,000." }
                    }
                }
                div {
                    label {
                        r#for: "timelock-hours",
                        class: "block text-sm font-medium text-gray-700",
                        "Timelock (Hours)"
                    }
                    div { class: "mt-1",
                        input {
                            r#type: "number",
                            min: "0",
                            step: "1",
                            max: "23",
                            name: "timelock-hours",
                            id: "timelock-hours",
                            class: hours_input_class,
                            placeholder: "0",
                            oninput: move |event| {
                                #[cfg(debug_assertions)]
                                trace!(% update_hour_var, event_value =% event.value(), "Set timelock hours");
                                validate_hours(&event.value());
                            },
                        }
                    }
                    if *hours_has_error.read() {
                        p { class: "mt-2 text-xs text-red-600", "Hours should be between 0 and 23." }
                    }
                }
            }
        }
    }
}

/// Escrow type input validation component.
#[component]
pub(crate) fn EscrowTypeInput(mut update_var: Signal<String>) -> Element {
    // Initialize the signal with "A" when the component is first created
    use_effect(move || {
        // Only set the default value if the current value is empty
        if update_var.read().is_empty() {
            update_var.set("A".to_string());
        }
    });

    #[allow(clippy::redundant_closure)]
    let current_value = use_memo(move || update_var());

    rsx! {
        div { class: "sm:col-span-3",
            label {
                r#for: "escrow-type",
                class: "block text-sm font-medium text-gray-700",
                "Escrow Type"
            }
            div { class: "mt-1",
                select {
                    id: "escrow-type",
                    name: "escrow-type",
                    class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                    oninput: move |event| {
                        #[cfg(debug_assertions)]
                        trace!(% update_var, event_value =% event.value(), "Set escrow type");
                        update_var.set(event.value());
                    },
                    value: current_value,
                    option { value: "A", "A - Collaborative (2-of-2)" }
                    option { value: "B", "B - Dispute: First Party + Arbitrator" }
                    option { value: "C", "C - Dispute: Second Party + Arbitrator" }
                }
            }
        }
    }
}

/// Nostr `nsec` input validation component.
#[component]
pub(crate) fn NsecInput(mut update_var: Signal<String>) -> Element {
    let mut has_error = use_signal(|| false);

    let mut validate_nsec = move |input: &str| {
        let is_valid = input.is_empty() || parse_nsec(input).is_ok();
        *has_error.write() = !is_valid && !input.is_empty();
        update_var.set(input.to_string());
    };

    let input_class = if *has_error.read() {
        "shadow-sm focus:ring-red-500 focus:border-red-500 block w-full sm:text-sm border-red-300 rounded-md p-2 border bg-red-50"
    } else {
        "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border"
    };

    rsx! {
        div { class: "sm:col-span-3",
            label {
                r#for: "nsec",
                class: "block text-sm font-medium text-gray-700",
                "Your Nostr Secret Key (nsec)"
            }
            div { class: "mt-1",
                input {
                    r#type: "password",
                    name: "nsec",
                    id: "nsec",
                    class: input_class,
                    placeholder: "nsec...",
                    oninput: move |event| {
                        validate_nsec(&event.value());
                    },
                }
            }
            if *has_error.read() {
                p { class: "mt-2 text-xs text-red-600",
                    "Invalid nsec format. Please enter a valid Nostr secret key."
                }
            } else {
                p { class: "mt-2 text-xs text-red-600",
                    "Your key is never stored or transmitted. All signing happens locally."
                }
            }
        }
    }
}

/// Transaction ID input validation component.
#[component]
pub(crate) fn TxidInput(mut update_var: Signal<String>, label: String, warning: String) -> Element {
    let mut has_error = use_signal(|| false);

    let mut validate_txid = move |input: &str| {
        let is_valid = input.is_empty() || input.parse::<Txid>().is_ok();
        *has_error.write() = !is_valid && !input.is_empty();
        update_var.set(input.to_string());
    };

    let input_class = if *has_error.read() {
        "shadow-sm focus:ring-red-500 focus:border-red-500 block w-full sm:text-sm border-red-300 rounded-md p-2 border bg-red-50"
    } else {
        "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border"
    };

    rsx! {
        div { class: "sm:col-span-3",
            label {
                r#for: "txid",
                class: "block text-sm font-medium text-gray-700",
                {label}
            }
            if !warning.is_empty() {
                p { class: "mt-2 text-xs text-red-600",
                    "Deposit a single transaction to the escrow address and inform the transaction ID.
                    This transaction will be used to fund the escrow address.
                    Note that it should be a coinjoin transaction between buyer and seller,
                    i.e. should have only one output: the escrow address with the whole total escrow amount."
                }
            }
            div { class: "mt-1",
                input {
                    r#type: "text",
                    name: "txid",
                    id: "txid",
                    class: input_class,
                    placeholder: "txid...",
                    oninput: move |event| {
                        #[cfg(debug_assertions)]
                        trace!(% update_var, event_value =% event.value(), "Set funding transaction ID");
                        validate_txid(&event.value());
                    },
                }
            }
            if *has_error.read() {
                p { class: "mt-2 text-xs text-red-600",
                    "Invalid transaction ID. Please enter a valid transaction ID."
                }
            }
        }
    }
}

/// Transaction input validation component.
#[component]
pub(crate) fn TransactionInput(
    mut update_var: Signal<String>,
    label: String,
    id: String,
) -> Element {
    let mut has_error = use_signal(|| false);

    let mut validate_transaction = move |input: &str| {
        // For empty inputs, don't show an error
        if input.is_empty() {
            *has_error.write() = false;
            update_var.set(input.to_string());
            return;
        }

        // Bitcoin transaction validation using `rust-bitcoin`
        let is_valid = consensus::encode::deserialize_hex::<Transaction>(input).is_ok();

        *has_error.write() = !is_valid;
        update_var.set(input.to_string());
    };

    let input_class = if *has_error.read() {
        "shadow-sm focus:ring-red-500 focus:border-red-500 block w-full sm:text-sm border-red-300 rounded-md p-2 border bg-red-50"
    } else {
        "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border"
    };

    rsx! {
        div { class: "sm:col-span-6",
            label {
                r#for: id.as_str(),
                class: "block text-sm font-medium text-gray-700",
                {label}
            }
            div { class: "mt-1",
                textarea {
                    id: id.as_str(),
                    name: id.as_str(),
                    rows: "4",
                    class: input_class,
                    placeholder: "Paste the transaction here...",
                    oninput: move |event| {
                        #[cfg(debug_assertions)]
                        trace!(% update_var, event_value =% event.value(), "Set transaction");
                        validate_transaction(&event.value());
                    },
                    value: update_var,
                }
            }
            if *has_error.read() {
                p { class: "mt-2 text-xs text-red-600",
                    "Invalid transaction format. The transaction should be a hexadecimal string."
                }
            }
        }
    }
}
/// Signature input validation component.
#[component]
pub(crate) fn SignatureInput(mut update_var: Signal<String>, label: String, id: String) -> Element {
    let mut has_error = use_signal(|| false);

    let mut validate_signature = move |input: &str| {
        // For empty inputs, don't show an error
        if input.is_empty() {
            *has_error.write() = false;
            update_var.set(input.to_string());
            return;
        }

        // Validate signature using `rust-bitcoin`
        let is_valid = input.parse::<schnorr::Signature>().is_ok();

        *has_error.write() = !is_valid;
        update_var.set(input.to_string());
    };

    let input_class = if *has_error.read() {
        "shadow-sm focus:ring-red-500 focus:border-red-500 block w-full sm:text-sm border-red-300 rounded-md p-2 border bg-red-50"
    } else {
        "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border"
    };

    rsx! {
        div { class: "sm:col-span-6",
            label {
                r#for: id.as_str(),
                class: "block text-sm font-medium text-gray-700",
                {label}
            }
            div { class: "mt-1",
                textarea {
                    id: id.as_str(),
                    name: id.as_str(),
                    rows: "4",
                    class: input_class,
                    placeholder: "Paste the signature here...",
                    oninput: move |event| {
                        #[cfg(debug_assertions)]
                        trace!(% update_var, event_value =% event.value(), "Set signature");
                        validate_signature(&event.value());
                    },
                    value: update_var,
                }
            }
            if *has_error.read() {
                p { class: "mt-2 text-xs text-red-600", "Invalid signature format." }
            }
        }
    }
}
/// Address input validation component.
#[component]
pub(crate) fn AddressInput(mut update_var: Signal<String>) -> Element {
    let mut has_error = use_signal(|| false);

    let mut validate_address = move |input: &str| {
        let is_valid = input.parse::<Address<_>>().is_ok()
            && input
                .parse::<Address<_>>()
                .and_then(|a| a.require_network(parse_network(&NETWORK.read()).unwrap()))
                .is_ok();

        *has_error.write() = !is_valid && !input.is_empty();
        update_var.set(input.to_string());
    };

    let input_class = if *has_error.read() {
        "shadow-sm focus:ring-red-500 focus:border-red-500 block w-full sm:text-sm border-red-300 rounded-md p-2 border bg-red-50"
    } else {
        "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border"
    };

    rsx! {
        div { class: "sm:col-span-3",
            label {
                r#for: "destination-address",
                class: "block text-sm font-medium text-gray-700",
                "Your Destination Address"
            }
            div { class: "mt-1",
                input {
                    r#type: "text",
                    name: "destination-address",
                    id: "destination-address",
                    class: input_class,
                    placeholder: "Enter your destination address...",
                    oninput: move |event| {
                        #[cfg(debug_assertions)]
                        trace!(% update_var, event_value =% event.value(), "Set address");
                        validate_address(&event.value());
                    },
                }
            }
            if *has_error.read() {
                p { class: "mt-2 text-xs text-red-600",
                    "Invalid Bitcoin address format. Please check and try again."
                }
            }
        }
    }
}
