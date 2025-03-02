//! Spend from resolution address component.

use dioxus::prelude::*;

#[cfg(debug_assertions)]
use dioxus::logger::tracing::trace;

use crate::{NETWORK, Route};

use super::{ContinueButton, CopyButton, Footer};

/// Spend from resolution address component.
#[component]
pub(crate) fn Spend() -> Element {
    let signed_transaction = use_signal(|| "Signed transaction will appear here...");
    rsx! {
        main { class: "max-w-7xl mx-auto py-6 sm:px-6 lg:px-8",
            div { class: "px-4 py-6 sm:px-0",
                h1 { class: "text-2xl font-bold text-gray-900 mb-6", "Spend from Resolution Address" }

                div { class: "bg-white shadow overflow-hidden sm:rounded-lg",
                    div { class: "px-4 py-5 sm:p-6",
                        div { class: "space-y-6",
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

                                div { class: "sm:col-span-3",
                                    label {
                                        r#for: "npub1",
                                        class: "block text-sm font-medium text-gray-700",
                                        "Your Nostr Public Key (npub)"
                                    }
                                    div { class: "mt-1",
                                        input {
                                            r#type: "text",
                                            name: "npub1",
                                            id: "npub1",
                                            class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                            placeholder: "npub...",
                                        }
                                    }
                                }

                                div { class: "sm:col-span-3",
                                    label {
                                        r#for: "txid",
                                        class: "block text-sm font-medium text-gray-700",
                                        "Escrow Resolution Transaction ID"
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
                                        "Total Locked Amount (BTC)"
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

                // Result Section (would be shown after button click)
                div { class: "mt-8 bg-white shadow overflow-hidden sm:rounded-lg",
                    div { class: "px-4 py-5 sm:p-6",
                        h3 { class: "text-lg leading-6 font-medium text-gray-900",
                            "Signed Transaction"
                        }

                        div { class: "mt-4 grid grid-cols-3 gap-y-6 gap-x-4 sm:grid-cols-2",
                            div { class: "col-span-3",
                                div { class: "mt-1",
                                    textarea {
                                        id: "signed-tx",
                                        readonly: "true",
                                        rows: "4",
                                        class: "shadow-sm block w-full sm:text-sm border-gray-300 rounded-md p-2 border bg-gray-50",
                                        placeholder: signed_transaction,
                                    }
                                }
                            }
                        }

                        div { class: "mt-5 flex flex-col space-y-3 sm:flex-row sm:space-y-0 sm:space-x-3",
                            CopyButton {
                                text: "Signature",
                                clipboard_text: signed_transaction,
                            }
                            ContinueButton {
                                to: Route::Broadcast {},
                                text: "Continue to Broadcast",
                            }
                        }
                    }
                }
            }
        }
        Footer {}
    }
}
