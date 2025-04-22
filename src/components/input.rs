//! Input Validation Components.

use bitcoin::{Address, Amount, FeeRate, Transaction, Txid, consensus};
use dioxus::prelude::*;

#[cfg(debug_assertions)]
use dioxus::logger::tracing::trace;
use secp256k1::schnorr;

use crate::{
    ESPLORA_ENDPOINT, NETWORK,
    esplora::FeeEstimate,
    util::{npub_to_address, parse_network, parse_npub, parse_nsec},
};

/// Nostr `npub` input validation component.
#[component]
pub(crate) fn NpubInput(
    mut update_var: Signal<String>,
    label: String,
    id: String,
    error: Signal<Option<String>>,
    required: Option<bool>,
) -> Element {
    let required = required.unwrap_or(false);

    let required_msg = match id.as_str() {
        "npub_1" => "First npub is required.",
        "npub_2" => "Second npub is required.",
        "npub_arbitrator" => "Arbitrator npub is required.",
        _ => "Npub is required.",
    };

    let invalid_msg = match id.as_str() {
        "npub_1" => "Invalid first npub format. Please enter a valid Nostr public key.",
        "npub_2" => "Invalid second npub format. Please enter a valid Nostr public key.",
        "npub_arbitrator" => {
            "Invalid arbitrator npub format. Please enter a valid Nostr public key."
        }
        _ => "Invalid npub format. Please enter a valid Nostr public key.",
    };

    let mut validate_npub = move |input: &str| {
        update_var.set(input.to_string());

        if input.is_empty() {
            if required {
                error.set(Some(required_msg.to_string()));
            } else {
                error.set(None);
            }
            return;
        } else if parse_npub(input).is_err() {
            error.set(Some(invalid_msg.to_string()));
        } else {
            error.set(None);
        };
    };

    let input_class = if error.read().is_some() {
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
            if let Some(error_msg) = error.read().as_ref() {
                p { class: "mt-2 text-xs text-red-600", "{error_msg}" }
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
    error: Signal<Option<String>>,
) -> Element {
    let required_msg = match id.as_str() {
        "npub_buyer" => "Buyer npub is required.",
        "npub_seller" => "Seller npub is required.",
        _ => "Npub is required.",
    };

    let invalid_msg = match id.as_str() {
        "npub_buyer" => "Invalid buyer npub format. Please enter a valid Nostr public key.",
        "npub_seller" => "Invalid seller npub format. Please enter a valid Nostr public key.",
        _ => "Invalid npub format. Please enter a valid Nostr public key.",
    };

    let mut validate_and_derive = move |input: &str| {
        let parsed_npub = parse_npub(input);
        let is_valid = !input.is_empty() && parsed_npub.is_ok();
        update_var.set(input.to_string());

        if input.is_empty() {
            error.set(Some(required_msg.to_string()));
            update_address.set(String::new());
            return;
        }

        if !is_valid {
            error.set(Some(invalid_msg.to_string()));
            update_address.set(String::new());
            return;
        }

        error.set(None);

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

    let input_class = if error.read().is_some() {
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

            if let Some(error_msg) = error.read().as_ref() {
                p { class: "mt-2 text-xs text-red-600", "{error_msg}" }
            }
        }
    }
}

/// Bitcoin BTC amount input validation component.
#[component]
pub(crate) fn BitcoinInput(
    mut update_var: Signal<String>,
    label: String,
    id: String,
    error: Signal<Option<String>>,
) -> Element {
    let required_msg = match id.as_str() {
        "amount_buyer" => "Buyer amount is required.",
        "amount_seller" => "Seller amount is required.",
        _ => "Amount is required.",
    };

    let invalid_msg = match id.as_str() {
        "amount_buyer" => "Buyer amount must be between 0.00000001 and 100 BTC.",
        "amount_seller" => "Seller amount must be between 0.00000001 and 100 BTC.",
        _ => "Amount must be between 0.00000001 and 100 BTC.",
    };

    let mut validate_amount = move |input: &str| {
        update_var.set(input.to_string());

        if input.is_empty() {
            error.set(Some(required_msg.to_string()));
            return;
        }

        match input.parse::<f64>() {
            Ok(amount)
                if amount >= 0.00000001 && amount <= 100.0 && Amount::from_btc(amount).is_ok() =>
            {
                error.set(None);
            }
            Ok(_) => {
                error.set(Some(invalid_msg.to_string()));
            }
            Err(_) => {
                error.set(Some("Invalid amount format.".to_string()));
            }
        }
    };

    let input_class = if error.read().is_some() {
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
            if let Some(error_msg) = error.read().as_ref()  {
                p { class: "mt-2 text-xs text-red-600", "{error_msg}" }
            }
        }
    }
}

/// Component to select the resolution transaction fee rate via input field or
/// dropdown with fees fetched from Esplora and their expected confirmation targets.
#[component]
pub(crate) fn FeeRateSelector(
    id: String,
    label_input: String,
    label_dropdown: String,
    mut update_var: Signal<String>,
    fee_estimates: Signal<Option<FeeEstimate>>,
    error: Signal<Option<String>>,
) -> Element {
    let required_msg = match id.as_str() {
        "fee" => "Fee rate is required.",
        _ => "This field is required.",
    };

    let invalid_msg = match id.as_str() {
        "fee" => "Fee rate must be a positive integer greater than zero.",
        _ => "Invalid fee rate.",
    };

    let mut validate_fee_rate = move |input: &str| {
        update_var.set(input.to_string());

        if input.is_empty() {
            error.set(Some(required_msg.to_string()));
            return;
        }

        match input.parse::<u64>() {
            Ok(rate) if rate > 0 && FeeRate::from_sat_per_vb(rate).is_some() => {
                error.set(None);
            }
            Ok(_) => {
                error.set(Some(invalid_msg.to_string()));
            }
            Err(_) => {
                error.set(Some("Invalid fee rate format.".to_string()));
            }
        }
    };

    let mut selected_target = use_signal(|| "3".to_string()); // Default to 3-block confirmation
    // Simple confirmation options - show just the blocks
    let confirmation_options = vec![
        ("1", "1 block"),
        ("3", "3 blocks"),
        ("6", "6 blocks"),
        ("9", "9 blocks"),
        ("12", "12 blocks"),
        ("15", "15 blocks"),
        ("24", "24 blocks"),
        ("144", "144 blocks"),
    ];

    // Update fee rate when selected target changes or when fee estimates are updated
    use_effect(move || {
        to_owned![update_var, fee_estimates, selected_target];

        if let Some(estimates) = fee_estimates.read().as_ref() {
            if let Some(fee) = estimates.get(&selected_target.read().parse::<u16>().unwrap_or(3)) {
                let rounded_fee = fee.ceil() as u64;
                update_var.set(rounded_fee.to_string());

                #[cfg(debug_assertions)]
                trace!(
                    "Updated fee rate to {} for target {} blocks",
                    rounded_fee,
                    selected_target.read()
                );
            }
        }
    });

    let input_class = "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border py-2 px-3";
    let select_class = "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border py-2 px-3";

    rsx! {
        div { class: "sm:col-span-3",
            div { class: "grid grid-cols-2 gap-10 w-full",
                div {
                    span { class: "block text-sm font-medium text-gray-700", {label_input} }
                    input {
                        r#type: "number",
                        min: "1",
                        step: "1",
                        name: id.as_str(),
                        id: id.as_str(),
                        class: input_class,
                        placeholder: "1",
                        value: "{update_var}",
                        oninput: move |event| {
                            #[cfg(debug_assertions)]
                            trace!(% update_var, event_value =% event.value(), "Set fee rate");
                            validate_fee_rate(&event.value());
                        },
                    }
                }
                div { class: "ml-3",
                    span { class: "block text-sm font-medium text-gray-700", {label_dropdown} }
                    select {
                        id: "{id}_selector",
                        class: select_class,
                        onchange: move |evt| {
                            selected_target.set(evt.value().to_string());
                        },
                        {
                            confirmation_options
                                .iter()
                                .map(|(value, label)| {
                                    let current_target = selected_target.read().clone();
                                    rsx! {
                                        option { value: "{value}", selected: current_target == *value, "{label}" }
                                    }
                                })
                        }
                    }
                }
            }
            if let Some(error_msg) = error.read().as_ref() {
                p { class: "mt-2 text-xs text-red-600", "{error_msg}" }
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
            "Testnet" => "https://mempool.space/testnet4/api",
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
    day_error: Signal<Option<String>>,
    hour_error: Signal<Option<String>>,
    required: Option<bool>,
) -> Element {
    let required = required.unwrap_or(false);

    let mut validate_days = move |input: &str| {
        update_day_var.set(input.to_string());

        if input.is_empty() {
            if required {
                day_error.set(Some("Timelock (days) is required.".to_string()));
            } else {
                day_error.set(None);
            }
            return;
        }

        match input.parse::<u32>() {
            Ok(days) if days <= 1_000 => {
                day_error.set(None);
            }
            Ok(_) => {
                day_error.set(Some("Days should be between 0 and 1,000.".to_string()));
            }
            Err(_) => {
                day_error.set(Some("Invalid days format.".to_string()));
            }
        }
    };

    let mut validate_hours = move |input: &str| {
        update_hour_var.set(input.to_string());

        if input.is_empty() {
            if required {
                hour_error.set(Some("Timelock (hours) is required.".to_string()));
            } else {
                hour_error.set(None);
            }
            return;
        }

        match input.parse::<u32>() {
            Ok(hours) if hours < 24 => {
                hour_error.set(None);
            }
            Ok(_) => {
                hour_error.set(Some("Hours should be between 0 and 23.".to_string()));
            }
            Err(_) => {
                hour_error.set(Some("Invalid hours format.".to_string()));
            }
        }
    };

    let days_input_class = if day_error.read().is_some() {
        "shadow-sm focus:ring-red-500 focus:border-red-500 block w-full sm:text-sm border-red-300 rounded-md p-2 border bg-red-50"
    } else {
        "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border"
    };

    let hours_input_class = if hour_error.read().is_some() {
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
                    if let Some(error_msg) = day_error.read().as_ref() {
                        p { class: "mt-2 text-xs text-red-600", "{error_msg}" }
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
                    if let Some(error_msg) = hour_error.read().as_ref() {
                        p { class: "mt-2 text-xs text-red-600", "{error_msg}" }
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
pub(crate) fn NsecInput(mut update_var: Signal<String>, error: Signal<Option<String>>) -> Element {
    let mut validate_nsec = move |input: &str| {
        update_var.set(input.to_string());

        if input.is_empty() {
            error.set(Some("Nsec is required.".to_string()));
        } else if parse_nsec(input).is_err() {
            error.set(Some(
                "Invalid nsec format. Please enter a valid Nostr secret key.".to_string(),
            ));
        } else {
            error.set(None);
        }
    };

    let input_class = if error.read().is_some() {
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
            if let Some(error_msg) = error.read().as_ref() {
                p { class: "mt-2 text-xs text-red-600", "{error_msg}" }
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
pub(crate) fn TxidInput(
    mut update_var: Signal<String>,
    label: String,
    warning: String,
    error: Signal<Option<String>>,
) -> Element {
    let mut validate_txid = move |input: &str| {
        update_var.set(input.to_string());

        if input.is_empty() {
            error.set(Some("Transaction ID is required.".to_string()));
        } else if input.parse::<Txid>().is_err() {
            error.set(Some(
                "Invalid transaction ID. Please enter a valid transaction ID.".to_string(),
            ));
        } else {
            error.set(None);
        }
    };

    let input_class = if error.read().is_some() {
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
            if let Some(error_msg) = error.read().as_ref() {
                p { class: "mt-2 text-xs text-red-600", "{error_msg}" }
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
    error: Signal<Option<String>>,
) -> Element {
    let required_msg = match id.as_str() {
        "signed-tx" => "Signed transaction is required.",
        "unsigned-tx" => "Unsigned transaction is required.",
        _ => "Transaction is required.",
    };

    let invalid_msg = match id.as_str() {
        "signed-tx" => {
            "Invalid signed transaction format. The transaction should be a hexadecimal string."
        }
        "unsigned-tx" => {
            "Invalid unsigned transaction format. The transaction should be a hexadecimal string."
        }
        _ => "Invalid transaction format.",
    };

    let mut validate_transaction = move |input: &str| {
        update_var.set(input.to_string());

        if input.is_empty() {
            error.set(Some(required_msg.to_string()));
        } else if consensus::encode::deserialize_hex::<Transaction>(input).is_err() {
            error.set(Some(invalid_msg.to_string()));
        } else {
            error.set(None);
        }
    };

    let input_class = if error.read().is_some() {
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
            if let Some(error_msg) = error.read().as_ref() {
                p { class: "mt-2 text-xs text-red-600", "{error_msg}" }
            }
        }
    }
}
/// Signature input validation component.
#[component]
pub(crate) fn SignatureInput(
    mut update_var: Signal<String>,
    label: String,
    id: String,
    error: Signal<Option<String>>,
    required: Option<bool>,
) -> Element {
    let required = required.unwrap_or(false);
    let required_msg = match id.as_str() {
        "signature1" => "First signature is required.",
        "signature2" => "Second signature is required.",
        "signaturearb" => "Arbitrator signature is required.",
        _ => "Signature is required.",
    };

    let invalid_msg = match id.as_str() {
        "signature1" => "Invalid first signature format.",
        "signature2" => "Invalid second signature format.",
        "signaturearb" => "Invalid arbitrator signature format.",
        _ => "Invalid signature format.",
    };

    let mut validate_signature = move |input: &str| {
        update_var.set(input.to_string());

        if input.is_empty() {
            if required {
                error.set(Some(required_msg.to_string()));
            } else {
                error.set(None);
            }
        } else if input.parse::<schnorr::Signature>().is_err() {
            error.set(Some(invalid_msg.to_string()));
        } else {
            error.set(None);
        }
    };

    let input_class = if error.read().is_some() {
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
             if let Some(error_msg) = error.read().as_ref() {
                p { class: "mt-2 text-xs text-red-600", "{error_msg}" }
            }
        }
    }
}
/// Address input validation component.
#[component]
pub(crate) fn AddressInput(
    mut update_var: Signal<String>,
    error: Signal<Option<String>>,
) -> Element {
    let required_msg = "Destination address is required.";
    let invalid_msg = "Invalid Bitcoin address format. Please check and try again.";

    let mut validate_address = move |input: &str| {
        update_var.set(input.to_string());

        if input.is_empty() {
            error.set(Some(required_msg.to_string()));
            return;
        }

        let is_valid = input.parse::<Address<_>>().is_ok()
            && input
                .parse::<Address<_>>()
                .and_then(|a| a.require_network(parse_network(&NETWORK.read()).unwrap()))
                .is_ok();

        if is_valid {
            error.set(None);
        } else {
            error.set(Some(invalid_msg.to_string()));
        }
    };

    let input_class = if error.read().is_some() {
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
            if let Some(error_msg) = error.read().as_ref() {
                p { class: "mt-2 text-xs text-red-600", "{error_msg}" }
            }
        }
    }
}

/// Vout input validation component (simple 0/1 option).
#[component]
pub(crate) fn VoutInput(mut update_var: Signal<String>, label: String, id: String) -> Element {
    // Initialize with default value "0" if empty
    use_effect(move || {
        if update_var.read().is_empty() {
            update_var.set("0".to_string());
        }
    });

    #[allow(clippy::redundant_closure)]
    let current_value = use_memo(move || update_var());

    rsx! {
        div { class: "sm:col-span-3",
            label {
                r#for: id.as_str(),
                class: "block text-sm font-medium text-gray-700",
                {label}
            }
            div { class: "mt-1",
                select {
                    id: id.as_str(),
                    name: id.as_str(),
                    class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                    oninput: move |event| {
                        #[cfg(debug_assertions)]
                        trace!(% update_var, event_value =% event.value(), "Set vout");
                        update_var.set(event.value());
                    },
                    value: current_value,
                    option { value: "0", "0" }
                    option { value: "1", "1" }
                }
            }
        }
    }
}
