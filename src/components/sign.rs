//! Sign escrow transaction component.

use dioxus::prelude::*;

#[cfg(debug_assertions)]
use dioxus::logger::tracing::info;

use crate::Route;

use super::{ContinueButton, CopyButton, Footer};

/// Sign escrow transaction component.
#[component]
pub(crate) fn Sign() -> Element {
    let signature = use_signal(|| "Signature will appear here...");
    rsx! {
        main { class: "max-w-7xl mx-auto py-6 sm:px-6 lg:px-8",
            div { class: "px-4 py-6 sm:px-0",
                h1 { class: "text-2xl font-bold text-gray-900 mb-6", "Sign Escrow" }

                div { class: "bg-white shadow overflow-hidden sm:rounded-lg",
                    div { class: "px-4 py-5 sm:p-6",
                        form {
                            onsubmit: move |event| {
                                #[cfg(debug_assertions)]
                                info!("Submitted! Event: {event:?}");
                                event.prevent_default();
                            },
                            div { class: "space-y-6",
                                div { class: "sm:col-span-6",
                                    label {
                                        r#for: "unsigned-tx",
                                        class: "block text-sm font-medium text-gray-700",
                                        "Unsigned Transaction String"
                                    }
                                    div { class: "mt-1",
                                        textarea {
                                            id: "unsigned-tx",
                                            name: "unsigned-tx",
                                            rows: "4",
                                            class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                            placeholder: "Paste the unsigned transaction here...",
                                        }
                                    }
                                }

                                div { class: "grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6",
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
                                                option { value: "A", "A - Collaborative (2-of-2)" }
                                                option { value: "B",
                                                    "B - Dispute: First Party + Arbitrator"
                                                }
                                                option { value: "C",
                                                    "C - Dispute: Second Party + Arbitrator"
                                                }
                                            }
                                        }
                                    }
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
                                            r#for: "txid",
                                            class: "block text-sm font-medium text-gray-700",
                                            "Funding Transaction ID"
                                        }
                                        div { class: "mt-1",
                                            input {
                                                r#type: "text",
                                                name: "txid",
                                                id: "txid",
                                                class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                                placeholder: "txid...",
                                            }
                                        }
                                    }

                                    div { class: "sm:col-span-3",
                                        label {
                                            r#for: "amount",
                                            class: "block text-sm font-medium text-gray-700",
                                            "Total Locked Escrow Amount (BTC)"
                                        }
                                        div { class: "mt-1",
                                            input {
                                                r#type: "number",
                                                step: "0.00000001",
                                                name: "amount",
                                                id: "amount",
                                                class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                                placeholder: "0.00000000",
                                            }
                                        }
                                    }

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
                                            }
                                        }
                                        p { class: "mt-2 text-xs text-red-600",
                                            "Your key is never stored or transmitted. All signing happens locally."
                                        }
                                    }
                                }

                                div {
                                    id: "arbitrator-section",
                                    class: "border-t border-gray-200 pt-6",
                                    h3 { class: "text-lg font-medium text-gray-900",
                                        "Arbitrator Details (for Dispute Resolution)"
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
                                        // TODO: Use PrimaryButton with a custom onclick
                                        button {
                                            r#type: "submit",
                                            class: "ml-3 inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
                                            "Sign Transaction"
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
                            "Signature"
                        }

                        div { class: "mt-5 border-t border-gray-200 pt-5",
                            div { class: "sm:col-span-6",
                                label {
                                    r#for: "signature",
                                    class: "block text-sm font-medium text-gray-500",
                                    "Your Signature"
                                }
                                div { class: "mt-1",
                                    textarea {
                                        id: "signature",
                                        readonly: "true",
                                        rows: "4",
                                        class: "shadow-sm block w-full sm:text-sm border-gray-300 rounded-md p-2 border bg-gray-50",
                                        placeholder: signature,
                                    }
                                }
                            }

                            div { class: "mt-5 flex",
                                CopyButton {
                                    text: "Signature",
                                    clipboard_text: signature,
                                }
                                ContinueButton {
                                    to: Route::Combine {},
                                    text: "Continue to Combine",
                                }
                            }
                        }
                    }
                }
            }
        }
        Footer {}
    }
}
