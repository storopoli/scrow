//! Sign escrow transaction component.

use bitcoin::{Amount, Transaction, TxOut, consensus};
use dioxus::prelude::*;

#[cfg(debug_assertions)]
use dioxus::logger::tracing::{info, trace};

use crate::{
    NETWORK, Route,
    scripts::escrow_address,
    sign::sign_escrow_tx,
    util::{
        days_to_blocks, hours_to_blocks, parse_escrow_type, parse_network, parse_npub, parse_nsec,
    },
};

use super::{ContinueButton, CopyButton, Footer, PrimaryButton};

/// Sign escrow transaction component.
#[component]
pub(crate) fn Sign() -> Element {
    let mut unsigned_tx = use_signal(String::new);
    let mut signature = use_signal(String::new);
    let mut escrow_type = use_signal(String::new);
    let mut npub_buyer = use_signal(String::new);
    let mut npub_seller = use_signal(String::new);
    let mut nsec = use_signal(String::new);
    let mut npub_arbitrator = use_signal(String::new);
    let mut btc_amount_total = use_signal(String::new);
    let mut timelock_days = use_signal(String::new);
    let mut timelock_hours = use_signal(String::new);
    let mut funding_txid = use_signal(String::new);
    let var_name = rsx! {
        main { class: "max-w-7xl mx-auto py-6 sm:px-6 lg:px-8",
            div { class: "px-4 py-6 sm:px-0",
                h1 { class: "text-2xl font-bold text-gray-900 mb-6", "Sign Escrow" }

                div { class: "bg-white shadow overflow-hidden sm:rounded-lg",
                    div { class: "px-4 py-5 sm:p-6",
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
                                        oninput: move |event| {
                                            #[cfg(debug_assertions)]
                                            trace!(% unsigned_tx, event_value =% event.value(), "Set unsigned transaction");
                                            unsigned_tx.set(event.value());
                                        },
                                        value: unsigned_tx,
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
                                                trace!(% escrow_type, event_value =% event.value(), "Set escrow type");
                                                escrow_type.set(event.value());
                                            },
                                            option { value: "A", "A - Collaborative (2-of-2)" }
                                            option { value: "B", "B - Dispute: First Party + Arbitrator" }
                                            option { value: "C", "C - Dispute: Second Party + Arbitrator" }
                                        }
                                    }
                                }
                                div { class: "sm:col-span-3",
                                    label {
                                        r#for: "npub_buyer",
                                        class: "block text-sm font-medium text-gray-700",
                                        "Buyer Nostr Public Key (npub)"
                                    }
                                    div { class: "mt-1",
                                        input {
                                            r#type: "text",
                                            name: "npub_buyer",
                                            id: "npub_buyer",
                                            class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                            placeholder: "npub...",
                                            oninput: move |event| {
                                                #[cfg(debug_assertions)]
                                                trace!(% npub_buyer, event_value =% event.value(), "Set buyer's npub");
                                                npub_buyer.set(event.value());
                                            },
                                        }
                                    }
                                }

                                div { class: "sm:col-span-3",
                                    label {
                                        r#for: "npub_seller",
                                        class: "block text-sm font-medium text-gray-700",
                                        "Seller Nostr Public Key (npub)"
                                    }
                                    div { class: "mt-1",
                                        input {
                                            r#type: "text",
                                            name: "npub_seller",
                                            id: "npub_seller",
                                            class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                            placeholder: "npub...",
                                            oninput: move |event| {
                                                #[cfg(debug_assertions)]
                                                trace!(% npub_seller, event_value =% event.value(), "Set seller's npub");
                                                npub_seller.set(event.value());
                                            },
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
                                            oninput: move |event| {
                                                #[cfg(debug_assertions)]
                                                trace!(
                                                    % funding_txid, event_value =% event.value(), "Set funding transaction ID"
                                                );
                                                funding_txid.set(event.value());
                                            },
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
                                            min: "0.00000001",
                                            max: "100.0",
                                            step: "0.00000001",
                                            name: "amount",
                                            id: "amount",
                                            class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                            placeholder: "0.00000000",
                                            oninput: move |event| {
                                                #[cfg(debug_assertions)]
                                                trace!(% btc_amount_total, event_value =% event.value(), "Set total BTC amount");
                                                btc_amount_total.set(event.value());
                                            },
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
                                                oninput: move |event| {
                                                    #[cfg(debug_assertions)]
                                                    trace!(% npub_arbitrator, event_value =% event.value(), "Set arbitrator's npub");
                                                    npub_arbitrator.set(event.value());
                                                },
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
                                                        step: "1",
                                                        name: "timelock-days",
                                                        id: "timelock-days",
                                                        class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                                        placeholder: "0",
                                                        oninput: move |event| {
                                                            #[cfg(debug_assertions)]
                                                            trace!(% timelock_days, event_value =% event.value(), "Set timelock days");
                                                            timelock_days.set(event.value());
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
                                                        max: "23",
                                                        step: "1",
                                                        name: "timelock-hours",
                                                        id: "timelock-hours",
                                                        class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                                        placeholder: "0",
                                                        oninput: move |event| {
                                                            #[cfg(debug_assertions)]
                                                            trace!(% timelock_hours, event_value =% event.value(), "Set timelock hours");
                                                            timelock_hours.set(event.value());
                                                        },
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            div { class: "pt-5",
                                div { class: "flex justify-end",
                                    PrimaryButton {
                                        onclick: move |_| {
                                            #[cfg(debug_assertions)]
                                            trace!(
                                                % npub_buyer, % npub_seller, % btc_amount_total, % NETWORK, %
                                                npub_arbitrator, % timelock_days, % timelock_hours, % escrow_type,
                                                "Clicked Generate Transaction"
                                            );
                                            let npub_buyer = parse_npub(&npub_buyer.read()).unwrap();
                                            let npub_seller = parse_npub(&npub_seller.read()).unwrap();
                                            let nsec = parse_nsec(&nsec.read()).unwrap();
                                            let escrow_type = parse_escrow_type(&escrow_type.read()).unwrap();
                                            let btc_amount_total = Amount::from_btc(
                                                    btc_amount_total.read().parse::<f64>().unwrap(),
                                                )
                                                .unwrap();
                                            let network = parse_network(&NETWORK.read()).unwrap();
                                            let unsigned_tx: Transaction = consensus::deserialize(
                                                    unsigned_tx.read().as_bytes(),
                                                )
                                                .unwrap();
                                            let signature_str = if !npub_arbitrator.read().is_empty() {
                                                #[cfg(debug_assertions)]
                                                trace!("dispute escrow address");
                                                let npub_arbitrator = parse_npub(&npub_arbitrator.read()).unwrap();
                                                let timelock_hours = hours_to_blocks(
                                                    timelock_hours.read().parse::<u32>().unwrap(),
                                                );
                                                let timelock_days = days_to_blocks(
                                                    timelock_days.read().parse::<u32>().unwrap(),
                                                );
                                                let timelock_duration = timelock_days + timelock_hours;
                                                let escrow_address = escrow_address(
                                                        &npub_buyer,
                                                        &npub_seller,
                                                        Some(&npub_arbitrator),
                                                        Some(timelock_duration),
                                                        network,
                                                    )
                                                    .unwrap();
                                                let prevout = TxOut {
                                                    value: btc_amount_total,
                                                    script_pubkey: escrow_address.script_pubkey(),
                                                };
                                                sign_escrow_tx(
                                                        &unsigned_tx,
                                                        0,
                                                        &nsec,
                                                        &npub_buyer,
                                                        &npub_seller,
                                                        Some(&npub_arbitrator),
                                                        Some(timelock_days + timelock_hours),
                                                        vec![prevout],
                                                        escrow_type,
                                                    )
                                                    .unwrap()
                                            } else {
                                                #[cfg(debug_assertions)]
                                                trace!("collaborative escrow address");
                                                let escrow_address = escrow_address(
                                                        &npub_buyer,
                                                        &npub_seller,
                                                        None,
                                                        None,
                                                        network,
                                                    )
                                                    .unwrap();
                                                let prevout = TxOut {
                                                    value: btc_amount_total,
                                                    script_pubkey: escrow_address.script_pubkey(),
                                                };
                                                sign_escrow_tx(
                                                        &unsigned_tx,
                                                        0,
                                                        &nsec,
                                                        &npub_buyer,
                                                        &npub_seller,
                                                        None,
                                                        None,
                                                        vec![prevout],
                                                        escrow_type,
                                                    )
                                                    .unwrap()
                                            };
                                            #[cfg(debug_assertions)]
                                            info!(% signature_str, "Generated signature");
                                            signature.set(signature_str.to_string());
                                        },
                                        text: "Sign Transaction",
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

                        div { class: "sm:col-span-6",
                            div { class: "mt-1",
                                textarea {
                                    id: "signature",
                                    readonly: "true",
                                    rows: "4",
                                    class: "shadow-sm block w-full sm:text-sm border-gray-300 rounded-md p-2 border bg-gray-50",
                                    placeholder: signature,
                                    value: signature,
                                }
                            }
                        }

                        div { class: "mt-5 flex flex-col space-y-3 sm:flex-row sm:space-y-0 sm:space-x-3",
                            CopyButton { text: "Signature", clipboard_text: signature }
                            ContinueButton {
                                to: Route::Combine {},
                                text: "Continue to Combine",
                            }
                        }
                    }
                }
            }
        }
        Footer {}
    };
    var_name
}
