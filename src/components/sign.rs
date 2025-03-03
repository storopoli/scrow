//! Sign escrow transaction component.

use bitcoin::hex::prelude::*;
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

use super::{
    BitcoinInput, ContinueButton, CopyButton, EscrowTypeInput, Footer, NetworkInput, NpubInput,
    NsecInput, PrimaryButton, TimelockInput,
};

/// Sign escrow transaction component.
#[component]
pub(crate) fn Sign() -> Element {
    let mut unsigned_tx = use_signal(String::new);
    let mut signature = use_signal(String::new);
    let escrow_type = use_signal(String::new);
    let npub_buyer = use_signal(String::new);
    let npub_seller = use_signal(String::new);
    let nsec = use_signal(String::new);
    let npub_arbitrator = use_signal(String::new);
    let amount_total = use_signal(String::new);
    let timelock_days = use_signal(String::new);
    let timelock_hours = use_signal(String::new);
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
                                NetworkInput { id: "network", label: "Bitcoin Network" }

                                EscrowTypeInput { update_var: escrow_type }

                                NpubInput {
                                    id: "npub_buyer",
                                    label: "Buyer Nostr Public Key (npub)",
                                    update_var: npub_buyer,
                                }

                                NpubInput {
                                    id: "npub_seller",
                                    label: "Seller Nostr Public Key (npub)",
                                    update_var: npub_seller,
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

                                BitcoinInput {
                                    id: "amount",
                                    label: "Total Locked Escrow Amount (BTC)",
                                    update_var: amount_total,
                                }

                                NsecInput { update_var: nsec }
                            }

                            div {
                                id: "arbitrator-section",
                                class: "border-t border-gray-200 pt-6",
                                h3 { class: "text-lg font-medium text-gray-900",
                                    "Arbitrator Details (for Dispute Resolution)"
                                }

                                div { class: "mt-4 grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6",

                                    NpubInput {
                                        id: "npub_arbitrator",
                                        label: "Arbitrator Nostr Public Key (npub)",
                                        update_var: npub_arbitrator,
                                    }

                                    TimelockInput {
                                        update_day_var: timelock_days,
                                        update_hour_var: timelock_hours,
                                    }
                                }
                            }

                            div { class: "pt-5",
                                div { class: "flex justify-end",
                                    PrimaryButton {
                                        onclick: move |_| {
                                            #[cfg(debug_assertions)]
                                            trace!(
                                                % npub_buyer, % npub_seller, % amount_total, % NETWORK, % npub_arbitrator, %
                                                timelock_days, % timelock_hours, % escrow_type,
                                                "Clicked Generate Transaction"
                                            );
                                            let npub_buyer = parse_npub(&npub_buyer.read()).unwrap();
                                            let npub_seller = parse_npub(&npub_seller.read()).unwrap();
                                            let nsec = parse_nsec(&nsec.read()).unwrap();
                                            let escrow_type = parse_escrow_type(&escrow_type.read()).unwrap();
                                            let btc_amount_total = Amount::from_btc(
                                                    amount_total.read().parse::<f64>().unwrap(),
                                                )
                                                .unwrap();
                                            let network = parse_network(&NETWORK.read()).unwrap();
                                            let unsigned_tx: Transaction = consensus::deserialize(
                                                    Vec::from_hex(&unsigned_tx.read()).unwrap().as_ref(),
                                                )
                                                .unwrap();
                                            let signature_str = if !npub_arbitrator.read().is_empty() {
                                                #[cfg(debug_assertions)]
                                                trace!("dispute escrow sign");
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
                                                trace!("collaborative escrow sign");
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

                // Result Section (would be shown after button click)
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
                                    placeholder: "Signature will appear here...",
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
