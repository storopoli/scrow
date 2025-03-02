//! Spend from resolution address component.

use bitcoin::{Address, Amount, TxOut, Txid, consensus, hex::DisplayHex};
use dioxus::prelude::*;

#[cfg(debug_assertions)]
use dioxus::logger::tracing::trace;

use crate::{
    NETWORK, Route,
    sign::sign_resolution_tx,
    tx::resolution_tx,
    util::{P2TR_TX_WEIGHT_FUNDING, npub_to_address, parse_network, parse_npub, parse_nsec},
};

use super::{ContinueButton, CopyButton, Footer, PrimaryButton};

/// Spend from resolution address component.
#[component]
pub(crate) fn Spend() -> Element {
    let mut npub = use_signal(String::new);
    let mut escrow_txid = use_signal(String::new);
    let mut destination_address = use_signal(String::new);
    let mut btc_amount = use_signal(String::new);
    let mut fee_rate = use_signal(|| "1".to_string());
    let mut derived_address = use_signal(String::new);
    let mut nsec = use_signal(String::new);
    let mut signed_tx_str = use_signal(String::new);
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
                                            name: "npub",
                                            id: "npub",
                                            class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                            placeholder: "npub...",
                                            oninput: move |event| {
                                                #[cfg(debug_assertions)]
                                                trace!(% npub, event_value =% event.value(), "Set npub");
                                                npub.set(event.value());
                                                let parsed_npub = parse_npub(&npub.read()).unwrap();
                                                let parsed_network = parse_network(&NETWORK.read()).unwrap();
                                                let derived_address_str = npub_to_address(&parsed_npub, parsed_network)
                                                    .unwrap()
                                                    .to_string();
                                                derived_address.set(derived_address_str);
                                            },
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
                                            oninput: move |event| {
                                                #[cfg(debug_assertions)]
                                                trace!(% escrow_txid, event_value =% event.value(), "Set escrow's txid");
                                                escrow_txid.set(event.value());
                                            },
                                        }
                                    }
                                }

                                div { class: "sm: col-span-3",
                                    label {
                                        r#for: "destination-address",
                                        class: "block text-sm font-medium text-gray-700",
                                        "Your Destination Address"
                                    }
                                    div { class: "mt-1",
                                        input {
                                            id: "destination-address",
                                            r#type: "text",
                                            name: "txid",
                                            id: "txid",
                                            class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                            class: "mt-1 text-sm text-gray-900 break-all bg-gray-50 p-3 rounded",
                                            placeholder: "Enter your destination address...",
                                            oninput: move |event| {
                                                #[cfg(debug_assertions)]
                                                trace!(
                                                    % destination_address, event_value =% event.value(),
                                                    "Set destination address"
                                                );
                                                destination_address.set(event.value());
                                            },
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
                                            min: "0.00000001",
                                            max: "100.0",
                                            step: "0.00000001",
                                            name: "amount",
                                            id: "amount",
                                            class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                            placeholder: "0.00000000",
                                            oninput: move |event| {
                                                #[cfg(debug_assertions)]
                                                trace!(% btc_amount, event_value =% event.value(), "Set BTC amount");
                                                btc_amount.set(event.value());
                                            },
                                        }
                                    }
                                }

                                div { class: "sm:col-span-3",
                                    label {
                                        r#for: "fee",
                                        class: "block text-sm font-medium text-gray-700",
                                        "Fee rate (sats/vByte)"
                                    }
                                    div { class: "mt-1",
                                        input {
                                            r#type: "number",
                                            min: "1",
                                            step: "1",
                                            name: "fee",
                                            id: "fee",
                                            class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                            placeholder: "1",
                                            oninput: move |event| {
                                                #[cfg(debug_assertions)]
                                                trace!(% fee_rate, event_value =% event.value(), "Set fee rate");
                                                fee_rate.set(event.value());
                                            },
                                        }
                                    }
                                }
                                div { class: "sm: col-span-3",
                                    dt { class: "text-sm font-medium text-gray-900",
                                        "Your Resolution Address"
                                    }
                                    dd {
                                        id: "buyer-address",
                                        class: "mt-1 text-sm text-gray-900 break-all bg-gray-50 p-3 rounded",
                                        {
                                            if derived_address.read().is_empty() {
                                                "bc1p...".to_string()
                                            } else {
                                                derived_address.read().clone()
                                            }
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
                                            oninput: move |event| {
                                                nsec.set(event.value());
                                            },
                                        }
                                    }
                                    p { class: "mt-2 text-xs text-red-600",
                                        "Your key is never stored or transmitted. All signing happens locally."
                                    }
                                }
                            }

                            div { class: "pt-5",
                                div { class: "flex justify-end",
                                    PrimaryButton {
                                        onclick: move |_| {
                                            #[cfg(debug_assertions)]
                                            trace!(
                                                % npub, % btc_amount, % NETWORK, % escrow_txid, % derived_address,
                                                "Clicked Sign Transaction"
                                            );
                                            let nsec = parse_nsec(&nsec.read()).unwrap();
                                            let btc_amount = Amount::from_btc(btc_amount.read().parse::<f64>().unwrap())
                                                .unwrap();
                                            let network = parse_network(&NETWORK.read()).unwrap();
                                            let escrow_txid = escrow_txid.read().parse::<Txid>().unwrap();
                                            let derived_address = derived_address
                                                .read()
                                                .parse::<Address<_>>()
                                                .unwrap()
                                                .require_network(network)
                                                .unwrap();
                                            let destination_address = destination_address
                                                .read()
                                                .parse::<Address<_>>()
                                                .unwrap()
                                                .require_network(network)
                                                .unwrap();
                                            let fee_rate = fee_rate.read().parse::<u64>().unwrap();
                                            let fee = Amount::from_sat(fee_rate * P2TR_TX_WEIGHT_FUNDING);
                                            let unsigned_tx = resolution_tx(
                                                btc_amount,
                                                escrow_txid,
                                                &destination_address,
                                                fee,
                                            );
                                            #[cfg(debug_assertions)]
                                            trace!(
                                                unsigned_tx = % consensus::serialize(& unsigned_tx).as_hex(),
                                                "Created unsigned resolution transaction"
                                            );
                                            let prevout = TxOut {
                                                value: btc_amount,
                                                script_pubkey: derived_address.script_pubkey(),
                                            };
                                            let signed_tx = sign_resolution_tx(&unsigned_tx, &nsec, prevout);
                                            let signed_tx = consensus::serialize(&signed_tx).as_hex().to_string();
                                            trace!(% signed_tx, "Signed resolution transaction");
                                            signed_tx_str.set(signed_tx);
                                        },
                                        text: "Sign Transaction",
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
                                        placeholder: signed_tx_str,
                                    }
                                }
                            }
                        }

                        div { class: "mt-5 flex flex-col space-y-3 sm:flex-row sm:space-y-0 sm:space-x-3",
                            CopyButton {
                                text: "Signature",
                                clipboard_text: signed_tx_str,
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
