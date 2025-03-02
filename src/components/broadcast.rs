//! Broadcast escrow transaction component.

use dioxus::prelude::*;

#[cfg(debug_assertions)]
use dioxus::logger::tracing::trace;

use super::Footer;

/// Broadcast escrow transaction component.
#[component]
pub(crate) fn Broadcast() -> Element {
    let broadcasted_txid =
        use_signal(|| "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef");
    rsx! {
        main { class: "max-w-7xl mx-auto py-6 sm:px-6 lg:px-8",
            div { class: "px-4 py-6 sm:px-0",
                h1 { class: "text-2xl font-bold text-gray-900 mb-6", "Broadcast Transaction" }

                div { class: "bg-white shadow overflow-hidden sm:rounded-lg",
                    div { class: "px-4 py-5 sm:p-6",
                        div { class: "space-y-6",
                            div { class: "sm:col-span-6",
                                label {
                                    r#for: "signed-tx",
                                    class: "block text-sm font-medium text-gray-700",
                                    "Signed Transaction String"
                                }
                                div { class: "mt-1",
                                    textarea {
                                        id: "signed-tx",
                                        name: "signed-tx",
                                        rows: "4",
                                        class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                        placeholder: "Paste the signed transaction here...",
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
                            }

                            div { class: "pt-5",
                                div { class: "flex justify-end",
                                    // TODO: Use PrimaryButton with a custom onclick
                                    button {
                                        r#type: "submit",
                                        class: "ml-3 inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
                                        "Broadcast Transaction"
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
                            "Broadcast Result"
                        }

                        div { class: "mt-5 border-t border-gray-200 pt-5",
                            // Success State
                            div { class: "rounded-md bg-green-50 p-4",
                                div { class: "flex",
                                    div { class: "flex-shrink-0, text-green-50",
                                        svg {
                                            xmlns: "http://www.w3.org/2000/svg",
                                            width: "24",
                                            height: "24",
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            "stroke-width": "2",
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            class: "lucide lucide-check",

                                            path { d: "M20 6 9 17l-5-5" }
                                        }
                                    }
                                    div { class: "ml-3",
                                        h3 { class: "text-sm font-medium text-green-800",
                                            "Transaction Broadcasted Successfully"
                                        }
                                        div { class: "mt-2 text-sm text-green-700",
                                            p {
                                                "Transaction ID: "
                                                span { class: "font-mono break-all",
                                                    {broadcasted_txid}
                                                }
                                            }
                                        }
                                        div { class: "mt-4",
                                            div { class: "-mx-2 -my-1.5 flex",
                                                a {
                                                    href: "#", // This would be dynamically set based on the TX ID and network
                                                    target: "_blank",
                                                    class: "bg-green-50 px-2 py-1.5 rounded-md text-sm font-medium text-green-800 hover:bg-green-100 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-green-50 focus:ring-green-600",
                                                    "View on Block Explorer"
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Error State (hidden by default)
                            div { class: "rounded-md bg-red-50 p-4 hidden", // Use conditional rendering instead of 'hidden' class in Dioxus
                                div { class: "flex",
                                    div { class: "flex-shrink-0",
                                        svg {
                                            xmlns: "http://www.w3.org/2000/svg",
                                            width: "20",
                                            height: "20",
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            "stroke-width": "2",
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            class: "text-red-400 lucide lucide-circle-alert",
                                            circle { cx: "12", cy: "12", r: "10" }
                                            line {
                                                x1: "12",
                                                x2: "12",
                                                y1: "8",
                                                y2: "12",
                                            }
                                            line {
                                                x1: "12",
                                                x2: "12.01",
                                                y1: "16",
                                                y2: "16",
                                            }
                                        }
                                    }
                                    div { class: "ml-3",
                                        h3 { class: "text-sm font-medium text-red-800",
                                            "Broadcast Failed"
                                        }
                                        div { class: "mt-2 text-sm text-red-700",
                                            p { "Error message will appear here" }
                                        }
                                        div { class: "mt-4",
                                            div { class: "-mx-2 -my-1.5 flex",
                                                // TODO: Use PrimaryButton with a custom onclick
                                                button {
                                                    r#type: "button",
                                                    class: "bg-red-50 px-2 py-1.5 rounded-md text-sm font-medium text-red-800 hover:bg-red-100 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-red-50 focus:ring-red-600",
                                                    "Try Again"
                                                }
                                            }
                                        }
                                    }
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
