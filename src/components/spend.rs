//! Spend from resolution address component.

use bitcoin::{Address, Amount, TxOut, Txid, consensus, hex::DisplayHex};
use dioxus::prelude::*;

#[cfg(debug_assertions)]
use dioxus::logger::tracing::trace;

use crate::{
    NETWORK, Route,
    sign::sign_resolution_tx,
    tx::resolution_tx,
    util::{P2TR_TX_WEIGHT_FUNDING, parse_network, parse_nsec},
};

use super::{
    BitcoinInput, ContinueButton, CopyButton, FeeRateInput, Footer, NetworkInput,
    NpubInputDerivedAddress, PrimaryButton,
};

/// Spend from resolution address component.
#[component]
pub(crate) fn Spend() -> Element {
    let npub = use_signal(String::new);
    let mut escrow_txid = use_signal(String::new);
    let mut destination_address = use_signal(String::new);
    let amount = use_signal(String::new);
    let fee_rate = use_signal(|| "1".to_string());
    let derived_address = use_signal(String::new);
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
                                NetworkInput { id: "network", label: "Bitcoin Network" }

                                NpubInputDerivedAddress {
                                    id: "npub",
                                    label: "Your Nostr Public Key (npub)",
                                    update_var: npub,
                                    update_address: derived_address,
                                }

                                div { class: "sm:col-span-3",
                                    label {
                                        r#for: "escrow-txid",
                                        class: "block text-sm font-medium text-gray-700",
                                        "Escrow Resolution Transaction ID"
                                    }
                                    div { class: "mt-1",
                                        input {
                                            r#type: "text",
                                            name: "escrow-txid",
                                            id: "escrow-txid",
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
                                                trace!(
                                                    % destination_address, event_value =% event.value(),
                                                    "Set destination address"
                                                );
                                                destination_address.set(event.value());
                                            },
                                        }
                                    }
                                }

                                BitcoinInput {
                                    id: "amount",
                                    label: "Total Locked Amount (BTC)",
                                    update_var: amount,
                                }

                                FeeRateInput {
                                    id: "fee",
                                    label: "Fee rate (sats/vByte)",
                                    update_var: fee_rate,
                                }

                                div { class: "sm:col-span-3",
                                    label { class: "block text-sm font-medium text-gray-700",
                                        "Your Resolution Address"
                                    }
                                    div { class: "mt-1",
                                        div { class: "text-sm text-gray-900 break-all bg-gray-50 p-3 rounded",
                                            {
                                                if derived_address.read().to_string().is_empty() {
                                                    "bc1p...".to_string()
                                                } else {
                                                    derived_address.read().clone().to_string()
                                                }
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
                                                % npub, % amount, % NETWORK, % escrow_txid, % derived_address,
                                                "Clicked Sign Transaction"
                                            );
                                            let nsec = parse_nsec(&nsec.read()).unwrap();
                                            let btc_amount = Amount::from_btc(amount.read().parse::<f64>().unwrap())
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
                                            #[cfg(debug_assertions)]
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
                                        value: signed_tx_str,
                                    }
                                }
                            }
                        }

                        div { class: "mt-5 flex flex-col space-y-3 sm:flex-row sm:space-y-0 sm:space-x-3",
                            CopyButton {
                                text: "Transaction",
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
