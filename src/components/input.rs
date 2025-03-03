//! Input Validation Components.

use dioxus::prelude::*;

#[cfg(debug_assertions)]
use dioxus::logger::tracing::trace;

use crate::{
    NETWORK,
    util::{npub_to_address, parse_network, parse_npub},
};

/// Nostr `npub` input validation component.
#[component]
pub(crate) fn NpubInput(mut update_var: Signal<String>, label: String, id: String) -> Element {
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
                    class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                    placeholder: "npub...",
                    oninput: move |event| {
                        #[cfg(debug_assertions)]
                        trace!(% id, % update_var, event_value =% event.value(), "Set npub");
                        update_var.set(event.value());
                    },
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
) -> Element {
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
                    class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                    placeholder: "npub...",
                    oninput: move |event| {
                        #[cfg(debug_assertions)]
                        trace!(% id, % update_var, event_value =% event.value(), "Set npub");
                        update_var.set(event.value());
                        let parsed_npub = parse_npub(&update_var.read()).unwrap();
                        let parsed_network = parse_network(&NETWORK.read()).unwrap();
                        let derived_address_str = npub_to_address(&parsed_npub, parsed_network)
                            .unwrap()
                            .to_string();
                        #[cfg(debug_assertions)]
                        trace!(
                            % id, % derived_address_str, % update_address, event_value =% event.value(),
                            "Set derived address"
                        );
                        update_address.set(derived_address_str);
                    },
                }
            }
        }
    }
}

/// Bitcoin BTC amount input validation component.
#[component]
pub(crate) fn BitcoinInput(mut update_var: Signal<String>, label: String, id: String) -> Element {
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
                    class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                    placeholder: "0.00000000",
                    oninput: move |event| {
                        #[cfg(debug_assertions)]
                        trace!(% update_var, event_value =% event.value(), "Set seller's BTC amount");
                        update_var.set(event.value());
                    },
                }
            }
        }
    }
}

/// Fee rate input validation component.
#[component]
pub(crate) fn FeeRateInput(mut update_var: Signal<String>, label: String, id: String) -> Element {
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
                    class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                    placeholder: "1",
                    oninput: move |event| {
                        #[cfg(debug_assertions)]
                        trace!(% update_var, event_value =% event.value(), "Set seller's BTC amount");
                        update_var.set(event.value());
                    },
                }
            }
        }
    }
}

/// Network input validation component.
#[component]
pub(crate) fn NetworkInput(label: String, id: String) -> Element {
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
                        *NETWORK.write() = event.value();
                    },
                    value: NETWORK.read().clone(),
                    option { value: "Mainnet", "Mainnet" }
                    option { value: "Testnet", "Testnet" }
                    option { value: "Signet", "Signet" }
                    option { value: "Regtest", "Regtest" }
                }
            }
        }
    }
}
