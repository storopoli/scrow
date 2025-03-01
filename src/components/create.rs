//! Create escrow transaction component.

use dioxus::logger::tracing::info;
use dioxus::prelude::*;

use super::CopyButton;
use crate::Route;

/// Create escrow transaction component.
#[component]
pub(crate) fn Create() -> Element {
    let address = use_signal(|| "bc1p...".to_string());
    let transaction = use_signal(|| "Transaction data will appear here.");
    rsx! {
        main { class: "max-w-7xl mx-auto py-6 sm:px-6 lg:px-8",
            div { class: "px-4 py-6 sm:px-0",
                h1 { class: "text-2xl font-bold text-gray-900 mb-6", "Create Escrow" }

                div { class: "bg-white shadow overflow-hidden sm:rounded-lg",
                    div { class: "px-4 py-5 sm:p-6",
                        form { onsubmit: move |event| info!("Submitted! Event: {event:?}"),

                            div { class: "space-y-6",
                                div { class: "grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6",
                                    div { class: "sm:col-span-3",
                                        label {
                                            r#for: "npub1",
                                            class: "block text-sm font-medium text-gray-700",
                                            "Buyer Nostr Public Key (npub)"
                                        }
                                        div { class: "mt-1",
                                            input {
                                                r#type: "text",
                                                name: "npub1",
                                                id: "npub1",
                                                class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                                placeholder: "npub1...",
                                            }
                                        }
                                    }

                                    div { class: "sm:col-span-3",
                                        label {
                                            r#for: "npub2",
                                            class: "block text-sm font-medium text-gray-700",
                                            "Seller Nostr Public Key (npub)"
                                        }
                                        div { class: "mt-1",
                                            input {
                                                r#type: "text",
                                                name: "npub2",
                                                id: "npub2",
                                                class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                                placeholder: "npub2...",
                                            }
                                        }
                                    }

                                    div { class: "sm:col-span-3",
                                        label {
                                            r#for: "amount1",
                                            class: "block text-sm font-medium text-gray-700",
                                            "Buyer Escrow Amount (BTC)"
                                        }
                                        div { class: "mt-1",
                                            input {
                                                r#type: "number",
                                                step: "0.00000001",
                                                name: "amount1",
                                                id: "amount1",
                                                class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                                placeholder: "0.00000000",
                                            }
                                        }
                                    }

                                    div { class: "sm:col-span-3",
                                        label {
                                            r#for: "amount2",
                                            class: "block text-sm font-medium text-gray-700",
                                            "Seller Escrow Amount (BTC)"
                                        }
                                        div { class: "mt-1",
                                            input {
                                                r#type: "number",
                                                step: "0.00000001",
                                                name: "amount2",
                                                id: "amount2",
                                                class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                                placeholder: "0.00000000",
                                            }
                                        }
                                    }

                                    div { class: "sm:col-span-3",
                                        label {
                                            r#for: "network",
                                            class: "block text-sm font-medium text-gray-700",
                                            "Bitcoin Network"
                                        }
                                        div { class: "mt-1",
                                            select {
                                                id: "network",
                                                name: "network",
                                                class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                                option { value: "mainnet", "Mainnet" }
                                                option { value: "testnet", "Testnet" }
                                                option { value: "signet", "Signet" }
                                                option { value: "regtest", "Regtest" }
                                            }
                                        }
                                    }
                                }

                                div { class: "border-t border-gray-200 pt-6",
                                    h3 { class: "text-lg font-medium text-gray-900",
                                        "Optional Dispute Resolution"
                                    }

                                    div { class: "mt-4 grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6",
                                        div { class: "sm:col-span-3",
                                            label {
                                                r#for: "arbitrator",
                                                class: "block text-sm font-medium text-gray-700",
                                                "Arbitrator Nostr Public Key (npub)"
                                            }
                                            div { class: "mt-1",
                                                input {
                                                    r#type: "text",
                                                    name: "arbitrator",
                                                    id: "arbitrator",
                                                    class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                                    placeholder: "npub...",
                                                }
                                            }
                                        }

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
                                                            name: "timelock-days",
                                                            id: "timelock-days",
                                                            class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                                            placeholder: "0",
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
                                                            max: "23",
                                                            name: "timelock-hours",
                                                            id: "timelock-hours",
                                                            class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                                            placeholder: "0",
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                div { class: "pt-5",
                                    div { class: "flex justify-end",
                                        button {
                                            r#type: "submit",
                                            class: "ml-3 inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
                                            "Generate Escrow"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Result Section (would be shown after form submission)
                div { class: "mt-8 bg-white shadow overflow-hidden sm:rounded-lg",
                    div { class: "px-4 py-5 sm:p-6",
                        h3 { class: "text-lg leading-6 font-medium text-gray-900",
                            "Escrow Details"
                        }

                        div { class: "mt-5 border-t border-gray-200 pt-5",
                            dl { class: "grid grid-cols-1 gap-x-4 gap-y-6 sm:grid-cols-2",
                                div { class: "sm:col-span-2",
                                    dt { class: "text-sm font-medium text-gray-500",
                                        "Deposit Address"
                                    }
                                    dd {
                                        id: "escrow-address",
                                        class: "mt-1 text-sm text-gray-900 break-all bg-gray-50 p-3 rounded",
                                        {address}
                                    }
                                }

                                div { class: "sm:col-span-2",
                                    dt { class: "text-sm font-medium text-gray-500",
                                        "Unsigned Escrow Resolution Transaction"
                                    }
                                    dd { class: "mt-1 text-sm text-gray-900",
                                        textarea {
                                            id: "escrow-transaction",
                                            readonly: "true",
                                            class: "w-full h-32 p-3 border border-gray-300 rounded-md bg-gray-50 focus:ring-indigo-500 focus:border-indigo-500",
                                            placeholder: "Transaction data will appear here...",
                                        }
                                    }
                                }
                            }

                            div { class: "mt-5 flex",
                                CopyButton { text: "Address", clipboard_text: address }
                                CopyButton {
                                    text: "Transaction",
                                    clipboard_text: transaction,
                                }
                                Link {
                                    to: Route::Home {}, // TODO: Replace with Route::Sign {} when available
                                    class: "ml-3 inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
                                    "Continue to Sign"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
