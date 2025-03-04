//! Input Validation Components.

use dioxus::prelude::*;

#[cfg(debug_assertions)]
use dioxus::logger::tracing::trace;

use crate::{
    ESPLORA_ENDPOINT, NETWORK,
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
    col_span: u8,
) -> Element {
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

/// Esplora backend input validation component.
#[component]
pub(crate) fn EsploraInput() -> Element {
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
                    class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                    placeholder: "https://mempool.space/api",
                    value: ESPLORA_ENDPOINT.read().clone(),
                    oninput: move |event| {
                        #[cfg(debug_assertions)]
                        trace!(% ESPLORA_ENDPOINT, event_value =% event.value(), "Set Eslora endpoint");
                        *ESPLORA_ENDPOINT.write() = event.value();
                    },
                }
            }
            p { class: "mt-2 text-xs text-gray-500", "Default for mainnet: https://mempool.space/api" }
        }
    }
}
/// Timelock input validation component.
#[component]
pub(crate) fn TimelockInput(
    mut update_day_var: Signal<String>,
    mut update_hour_var: Signal<String>,
) -> Element {
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
                            class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                            placeholder: "0",
                            oninput: move |event| {
                                #[cfg(debug_assertions)]
                                trace!(% update_day_var, event_value =% event.value(), "Set timelock days");
                                update_day_var.set(event.value());
                            },
                        }
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
                            class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                            placeholder: "0",
                            oninput: move |event| {
                                #[cfg(debug_assertions)]
                                trace!(% update_hour_var, event_value =% event.value(), "Set timelock hours");
                                update_hour_var.set(event.value());
                            },
                        }
                    }
                }
            }
        }
    }
}

/// Escrow type input validation component.
#[component]
pub(crate) fn EscrowTypeInput(mut update_var: Signal<String>) -> Element {
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
                    class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                    placeholder: "nsec...",
                    oninput: move |event| {
                        update_var.set(event.value());
                    },
                }
            }
            p { class: "mt-2 text-xs text-red-600",
                "Your key is never stored or transmitted. All signing happens locally."
            }
        }
    }
}

/// Transaction ID input validation component.
#[component]
pub(crate) fn TxidInput(mut update_var: Signal<String>, label: String, warning: String) -> Element {
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
                    class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                    placeholder: "txid...",
                    oninput: move |event| {
                        #[cfg(debug_assertions)]
                        trace!(% update_var, event_value =% event.value(), "Set funding transaction ID");
                        update_var.set(event.value());
                    },
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
    rsx! {
        div { class: "sm:col-span-6",
            label {
                r#for: "unsigned-tx",
                class: "block text-sm font-medium text-gray-700",
                {label}
            }
            div { class: "mt-1",
                textarea {
                    id: id.as_str(),
                    name: id.as_str(),
                    rows: "4",
                    class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                    placeholder: "Paste the transaction here...",
                    oninput: move |event| {
                        #[cfg(debug_assertions)]
                        trace!(% update_var, event_value =% event.value(), "Set transaction");
                        update_var.set(event.value());
                    },
                    value: update_var,
                }
            }
        }
    }
}

/// Signature input validation component.
#[component]
pub(crate) fn SignatureInput(mut update_var: Signal<String>, label: String, id: String) -> Element {
    rsx! {
        div { class: "sm:col-span-6",
            label {
                r#for: id.as_str(),
                class: "block text-sm font-medium text-gray-700",
                {label}
            }
            div { class: "mt-1",
                textarea {
                    id: "unsigned-tx",
                    name: "unsigned-tx",
                    rows: "4",
                    class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                    placeholder: "Paste the signature here...",
                    oninput: move |event| {
                        #[cfg(debug_assertions)]
                        trace!(% update_var, event_value =% event.value(), "Set signature");
                        update_var.set(event.value());
                    },
                    value: update_var,
                }
            }
        }
    }
}

/// Address input validation component.
#[component]
pub(crate) fn AddressInput(mut update_var: Signal<String>) -> Element {
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
                    class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                    placeholder: "Enter your destination address...",
                    oninput: move |event| {
                        #[cfg(debug_assertions)]
                        trace!(% update_var, event_value =% event.value(), "Set address");
                        update_var.set(event.value());
                    },
                }
            }
        }
    }
}
