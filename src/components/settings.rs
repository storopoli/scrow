//! Settings component.

use dioxus::prelude::*;

#[cfg(debug_assertions)]
use dioxus::logger::tracing::trace;

use crate::{ESPLORA_ENDPOINT, NETWORK};

use super::Footer;

/// Settings component.
#[component]
pub(crate) fn Settings() -> Element {
    // Read the current values from global state
    rsx! {
        main { class: "max-w-7xl mx-auto py-6 sm:px-6 lg:px-8",
            div { class: "px-4 py-6 sm:px-0",
                h1 { class: "text-2xl font-bold text-gray-900 mb-6", "Settings" }

                div { class: "bg-white shadow overflow-hidden sm:rounded-lg",
                    div { class: "px-4 py-5 sm:p-6",
                        div { class: "space-y-6",
                            div { class: "grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6",
                                div { class: "sm:col-span-3",
                                    label {
                                        r#for: "network",
                                        class: "block text-sm font-medium text-gray-700",
                                        "Default Bitcoin Network"
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
                                    p { class: "mt-2 text-xs text-gray-500",
                                        "Default for mainnet: https://mempool.space/api"
                                    }
                                }
                            }

                            div { class: "pt-5",
                                div { class: "flex justify-end space-x-3",
                                    // TODO: Use SecondaryButton with a custom onclick
                                    button {
                                        r#type: "button",
                                        class: "inline-flex justify-center py-2 px-4 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
                                        onclick: move |_| {
                                            *NETWORK.write() = "Mainnet".to_string();
                                            *ESPLORA_ENDPOINT.write() = "https://mempool.space/api".to_string();
                                        },
                                        "Restore Defaults"
                                    }
                                    // TODO: Use PrimaryButton with a custom onclick
                                    button {
                                        r#type: "button",
                                        class: "inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
                                        "Save Settings"
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "mt-8 bg-white shadow overflow-hidden sm:rounded-lg",
                    div { class: "px-4 py-5 sm:p-6",
                        h3 { class: "text-lg leading-6 font-medium text-gray-900",
                            "About Satoshi Escrow"
                        }

                        div { class: "mt-2 max-w-xl text-sm text-gray-500",
                            p { "Version: 0.1.0" }
                            p { class: "mt-2",
                                "A Bitcoin non-custodial peer-to-peer dispute resolution tool. All code is open source and runs entirely in your browser."
                            }
                        }

                        div { class: "mt-5",
                            a {
                                href: "https://github.com/storopoli/scrow",
                                target: "_blank",
                                class: "inline-flex items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
                                // GitHub icon SVG
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "16",
                                    height: "16",
                                    view_box: "0 0 24 24",
                                    fill: "currentColor",
                                    class: "mr-2",
                                    path { d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" }
                                }
                                "View on GitHub"
                            }
                        }
                    }
                }
            }
        }
        Footer {}
    }
}
