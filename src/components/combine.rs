//! Combine escrow signatures component.

use dioxus::prelude::*;

#[cfg(debug_assertions)]
use dioxus::logger::tracing::info;

use crate::Route;

use super::{ContinueButton, CopyButton, Footer};

/// Combine escrow transaction component.
#[component]
pub(crate) fn Combine() -> Element {
    let signed_transaction = use_signal(|| "Signed transaction will appear here...");
    rsx! {
        main { class: "max-w-7xl mx-auto py-6 sm:px-6 lg:px-8",
            div { class: "px-4 py-6 sm:px-0",
                h1 { class: "text-2xl font-bold text-gray-900 mb-6", "Combine Signatures" }

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
                                            r#for: "npub1",
                                            class: "block text-sm font-medium text-gray-700",
                                            "First Party Nostr Public Key (npub)"
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
                                            "Second Party Nostr Public Key (npub)"
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
                                            r#for: "signature1",
                                            class: "block text-sm font-medium text-gray-700",
                                            "First Signature"
                                        }
                                        div { class: "mt-1",
                                            textarea {
                                                id: "signature1",
                                                name: "signature1",
                                                rows: "2",
                                                class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                                placeholder: "Paste the first signature here...",
                                            }
                                        }
                                    }

                                    div { class: "sm:col-span-3",
                                        label {
                                            r#for: "signature2",
                                            class: "block text-sm font-medium text-gray-700",
                                            "Second Signature"
                                        }
                                        div { class: "mt-1",
                                            textarea {
                                                id: "signature2",
                                                name: "signature2",
                                                rows: "2",
                                                class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                                placeholder: "Paste the second signature here...",
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

                                        div { class: "sm:col-span-3",
                                            label {
                                                r#for: "signaturearb",
                                                class: "block text-sm font-medium text-gray-700",
                                                "Arbitrator Signature"
                                            }
                                            div { class: "mt-1",
                                                textarea {
                                                    id: "signaturearb",
                                                    name: "signaturearb",
                                                    rows: "2",
                                                    class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                                    placeholder: "Paste the arbitrator signature here...",
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
                                            "Combine Signatures"
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
                            "Signed Transaction"
                        }

                        div { class: "mt-5 border-t border-gray-200 pt-5",
                            div { class: "sm:col-span-6",
                                label {
                                    r#for: "signed-tx",
                                    class: "block text-sm font-medium text-gray-500",
                                    "Signed Transaction"
                                }
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

                            div { class: "mt-5 flex",
                                CopyButton {
                                    text: "Transaction",
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
        }
        Footer {}
    }
}
