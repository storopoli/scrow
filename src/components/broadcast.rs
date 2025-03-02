//! Broadcast escrow transaction component.

use bitcoin::hex::prelude::*;
use bitcoin::{Transaction, consensus};
use dioxus::prelude::*;

#[cfg(debug_assertions)]
use dioxus::logger::tracing::{info, trace};

use crate::esplora::{broadcast_transaction, create_client};
use crate::{ESPLORA_ENDPOINT, NETWORK};

use super::{Footer, PrimaryButton};

/// Broadcast escrow transaction component.
#[component]
pub(crate) fn Broadcast() -> Element {
    let mut signed_tx = use_signal(String::new);
    let mut broadcast_result_str = use_signal(String::new);
    let mut broadcasted_txid = use_signal(String::new);
    let esplora_base_url = use_memo(move || {
        let esplora_endpoint = ESPLORA_ENDPOINT.read().clone();
        let break_points = esplora_endpoint.split("api").collect::<Vec<&str>>();
        break_points
            .first()
            .map(|&url| url.to_string())
            .unwrap_or_default()
    });
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
                                        oninput: move |event| {
                                            #[cfg(debug_assertions)]
                                            trace!(% signed_tx, event_value =% event.value(), "Set signed transaction");
                                            signed_tx.set(event.value());
                                        },
                                        value: signed_tx,
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

                            div { class: "pt-5",
                                div { class: "flex justify-end",
                                    PrimaryButton {
                                        onclick: move |_| {
                                            #[cfg(debug_assertions)]
                                            info!(% ESPLORA_ENDPOINT, "Created esplora client");
                                            let esplora_client = create_client(&ESPLORA_ENDPOINT.read()).unwrap();
                                            let signed_tx: Transaction = consensus::deserialize(
                                                    Vec::from_hex(&signed_tx.read()).unwrap().as_ref(),
                                                )
                                                .unwrap();
                                            let txid = signed_tx.compute_txid();
                                            broadcasted_txid.set(txid.to_string());
                                            spawn(async move {
                                                let broadcast_result = broadcast_transaction(&esplora_client, &signed_tx)
                                                    .await;
                                                #[cfg(debug_assertions)]
                                                info!(? broadcast_result, "broadcast_result");
                                                match broadcast_result {
                                                    Ok(_) => {
                                                        #[cfg(debug_assertions)]
                                                        info!(% txid, "Transaction broadcasted successfully");
                                                        broadcast_result_str.set("Success".to_string());
                                                    }
                                                    Err(err) => {
                                                        #[cfg(debug_assertions)]
                                                        trace!(% txid, ? err, "Transaction broadcast failed");
                                                        let error_string = format!(
                                                            "Error broadcasting transaction: {}",
                                                            err,
                                                        );
                                                        broadcast_result_str.set(error_string);
                                                    }
                                                }
                                            });
                                        },
                                        text: "Broadcast Transaction",
                                    }
                                }
                            }
                        }
                    }
                }

                // Success State
                if !broadcasted_txid.read().is_empty()
                    && broadcast_result_str.read().starts_with("Success")
                {
                    // Result Section
                    div { class: "mt-8 bg-white shadow overflow-hidden sm:rounded-lg",
                        div { class: "px-4 py-5 sm:p-6",

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
                                                    href: format!("{}tx/{}", esplora_base_url.read(), broadcasted_txid.read()),
                                                    target: "_blank",
                                                    class: "bg-green-50 px-2 py-1.5 rounded-md text-sm font-medium text-green-800 hover:bg-green-100 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-green-50 focus:ring-green-600",
                                                    "View on Block Explorer"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if !broadcasted_txid.read().is_empty()
                    && broadcast_result_str.read().starts_with("Error")
                {
                    // Result Section
                    div { class: "mt-8 bg-white shadow overflow-hidden sm:rounded-lg",
                        div { class: "px-4 py-5 sm:p-6",

                            div { class: "rounded-md bg-red-50 p-4 ",
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
                                            p { {broadcast_result_str} }
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
