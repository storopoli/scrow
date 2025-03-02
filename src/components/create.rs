//! Create escrow transaction component.

use bitcoin::{Amount, Txid, consensus, hex::DisplayHex};
use dioxus::prelude::*;

#[cfg(debug_assertions)]
use dioxus::logger::tracing::trace;

use crate::{
    NETWORK, Route,
    scripts::escrow_address,
    tx::escrow_tx,
    util::{P2TR_TX_WEIGHT_FUNDING, hours_to_blocks, parse_network, parse_npub},
};

use super::{ContinueButton, CopyButton, Footer, PrimaryButton};

/// Create escrow transaction component.
#[component]
pub(crate) fn Create() -> Element {
    let mut npub_buyer = use_signal(|| "Buyer Nostr Public Key (npub)".to_string());
    let mut npub_seller = use_signal(|| "Seller Nostr Public Key (npub)".to_string());
    let mut npub_arbitrator = use_signal(String::new);
    let mut btc_amount_buyer = use_signal(String::new);
    let mut btc_amount_seller = use_signal(String::new);
    let mut fee_rate = use_signal(|| "1".to_string());
    let mut timelock_days = use_signal(String::new);
    let mut timelock_hours = use_signal(String::new);
    let mut funding_txid = use_signal(String::new);
    let mut escrow_address_str = use_signal(|| "bc1p...".to_string());
    let mut escrow_transaction = use_signal(|| "Transaction data will appear here.".to_string());
    rsx! {
        main { class: "max-w-7xl mx-auto py-6 sm:px-6 lg:px-8",
            div { class: "px-4 py-6 sm:px-0",
                h1 { class: "text-2xl font-bold text-gray-900 mb-6", "Create Escrow" }

                div { class: "bg-white shadow overflow-hidden sm:rounded-lg",
                    div { class: "px-4 py-5 sm:p-6",
                        div { class: "space-y-6",
                            div { class: "grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6",
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
                                        r#for: "amount_buyer",
                                        class: "block text-sm font-medium text-gray-700",
                                        "Buyer Escrow Amount (BTC)"
                                    }
                                    div { class: "mt-1",
                                        input {
                                            r#type: "number",
                                            min: "0.00000001",
                                            step: "0.00000001",
                                            name: "amount_buyer",
                                            id: "amount_buyer",
                                            class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                            placeholder: "0.00000000",
                                            oninput: move |event| {
                                                #[cfg(debug_assertions)]
                                                trace!(
                                                    % btc_amount_buyer, event_value =% event.value(), "Set buyer's BTC amount"
                                                );
                                                btc_amount_buyer.set(event.value());
                                            },
                                        }
                                    }
                                }

                                div { class: "sm:col-span-3",
                                    label {
                                        r#for: "amount_seller",
                                        class: "block text-sm font-medium text-gray-700",
                                        "Seller Escrow Amount (BTC)"
                                    }
                                    div { class: "mt-1",
                                        input {
                                            r#type: "number",
                                            min: "0.00000001",
                                            step: "0.00000001",
                                            name: "amount_seller",
                                            id: "amount_seller",
                                            class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                            placeholder: "0.00000000",
                                            oninput: move |event| {
                                                #[cfg(debug_assertions)]
                                                trace!(
                                                    % btc_amount_seller, event_value =% event.value(), "Set seller's BTC amount"
                                                );
                                                btc_amount_seller.set(event.value());
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

                            div { class: "border-t border-gray-200 pt-6",
                                h3 { class: "text-lg font-medium text-gray-900",
                                    "Optional Dispute Resolution"
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
                                                        step: "1",
                                                        max: "23",
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

                            div { class: "border-t border-gray-200 pt-6",
                                div { class: "sm:col-span-2",
                                    dt { class: "text-lg font-medium text-gray-900",
                                        "Deposit Address"
                                    }
                                    dd {
                                        id: "escrow-address",
                                        class: "mt-1 text-md text-gray-900 break-all bg-gray-50 p-3 rounded",
                                        {escrow_address_str}
                                    }
                                }
                            }


                            div { class: "mt-5 flex",
                                CopyButton {
                                    text: "Address",
                                    clipboard_text: escrow_address_str,
                                }
                                PrimaryButton {
                                    onclick: move |_| {
                                        #[cfg(debug_assertions)]
                                        trace!(
                                            % npub_buyer, % npub_seller, % btc_amount_buyer, % btc_amount_seller, %
                                            fee_rate, % NETWORK, % npub_arbitrator, % timelock_days, % timelock_hours,
                                            "Clicked Generate Address"
                                        );
                                        let npub_buyer = parse_npub(&npub_buyer.read()).unwrap();
                                        let npub_seller = parse_npub(&npub_seller.read()).unwrap();
                                        let network = parse_network(NETWORK.read().clone()).unwrap();
                                        let resolved_escrow_address = if !npub_arbitrator.read().is_empty() {
                                            #[cfg(debug_assertions)]
                                            trace!("dispute escrow address");
                                            let npub_arbitrator = parse_npub(&npub_arbitrator.read()).unwrap();
                                            let timelock_hours = hours_to_blocks(
                                                timelock_hours.read().parse::<u32>().unwrap(),
                                            );
                                            let timelock_days = hours_to_blocks(
                                                timelock_days.read().parse::<u32>().unwrap(),
                                            );
                                            escrow_address(
                                                    &npub_buyer,
                                                    &npub_seller,
                                                    Some(&npub_arbitrator),
                                                    Some(timelock_days + timelock_hours),
                                                    network,
                                                )
                                                .unwrap()
                                                .to_string()
                                        } else {
                                            #[cfg(debug_assertions)]
                                            trace!("collaborative escrow address");
                                            escrow_address(&npub_buyer, &npub_seller, None, None, network)
                                                .unwrap()
                                                .to_string()
                                        };
                                        #[cfg(debug_assertions)]
                                        trace!(% resolved_escrow_address, "Derived escrow address");
                                        escrow_address_str.set(resolved_escrow_address);
                                    },
                                    text: "Generate Address",
                                }
                            }
                        }
                    }
                }

                // Result Section (would be shown after button click)
                div { class: "mt-8 bg-white shadow overflow-hidden sm:rounded-lg",
                    div { class: "px-4 py-5 sm:p-6",
                        h3 { class: "text-lg leading-6 font-medium text-gray-900",
                            "Escrow Details"
                        }
                        div { class: "mt-4 grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6",
                            div { class: "sm:col-span-3",
                                label {
                                    r#for: "funding_txid",
                                    class: "block text-sm font-medium text-gray-700",
                                    "Escrow funding Transaction ID"
                                }
                                p { class: "mt-2 text-xs text-red-600",
                                    "Deposit a single transaction to the escrow address and inform the transaction ID.
                                                                    This transaction will be used to fund the escrow address.
                                                                    Note that it should be a coinjoin transaction between buyer and seller,
                                                                    i.e. should have only one output: the escrow address with the whole total escrow amount."
                                }
                                div { class: "mt-1",
                                    input {
                                        r#type: "text",
                                        name: "funding_txid",
                                        id: "funding_txid",
                                        class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                        placeholder: "txid...",
                                        oninput: move |event| {
                                            #[cfg(debug_assertions)]
                                            trace!(% funding_txid, event_value =% event.value(), "Set funding_txid");
                                            funding_txid.set(event.value());
                                        },
                                    }
                                }
                            }
                        }


                        div { class: "mt-5 border-t border-gray-200 pt-5",
                            dl { class: "grid grid-cols-1 gap-x-4 gap-y-6 sm:grid-cols-2",

                                div { class: "sm:col-span-2",
                                    dt { class: "text-sm font-medium text-gray-500",
                                        "Unsigned Escrow Resolution Transaction"
                                    }
                                    dd { class: "mt-1 text-sm text-gray-900",
                                        textarea {
                                            id: "escrow-transaction",
                                            readonly: "true",
                                            class: "w-full h-32 p-3 border border-gray-300 rounded-md bg-gray-50 focus:ring-indigo-500 focus:border-indigo-500",
                                            value: escrow_transaction,
                                        }
                                    }
                                }
                            }

                            div { class: "mt-5 flex",
                                CopyButton {
                                    text: "Transaction",
                                    clipboard_text: escrow_transaction,
                                }
                                PrimaryButton {
                                    onclick: move |_| {
                                        #[cfg(debug_assertions)]
                                        trace!(
                                            % npub_buyer, % npub_seller, % btc_amount_buyer, % btc_amount_seller, %
                                            fee_rate, % NETWORK, % npub_arbitrator, % timelock_days, % timelock_hours,
                                            "Clicked Generate Transaction"
                                        );
                                        let npub_buyer = parse_npub(&npub_buyer.read()).unwrap();
                                        let npub_seller = parse_npub(&npub_seller.read()).unwrap();
                                        let btc_amount_buyer = Amount::from_btc(
                                                btc_amount_buyer.read().parse::<f64>().unwrap(),
                                            )
                                            .unwrap();
                                        let btc_amount_seller = Amount::from_btc(
                                                btc_amount_seller.read().parse::<f64>().unwrap(),
                                            )
                                            .unwrap();
                                        let fee_rate = fee_rate.read().parse::<u64>().unwrap();
                                        let fee = Amount::from_sat(fee_rate * P2TR_TX_WEIGHT_FUNDING);
                                        let network = parse_network(NETWORK.read().clone()).unwrap();
                                        let funding_txid = funding_txid.read().parse::<Txid>().unwrap();
                                        let resolved_escrow_transaction = if !npub_arbitrator.read().is_empty() {
                                            #[cfg(debug_assertions)]
                                            trace!("dispute escrow address");
                                            let timelock_hours = hours_to_blocks(
                                                timelock_hours.read().parse::<u32>().unwrap(),
                                            );
                                            let timelock_days = hours_to_blocks(
                                                timelock_days.read().parse::<u32>().unwrap(),
                                            );
                                            let escrow_tx = escrow_tx(
                                                    &npub_buyer,
                                                    &npub_seller,
                                                    Some(timelock_days + timelock_hours),
                                                    btc_amount_buyer,
                                                    btc_amount_seller,
                                                    funding_txid,
                                                    fee,
                                                    network,
                                                )
                                                .unwrap();
                                            consensus::serialize(&escrow_tx).as_hex().to_string()
                                        } else {
                                            #[cfg(debug_assertions)]
                                            trace!("collaborative escrow address");
                                            let escrow_tx = escrow_tx(
                                                    &npub_buyer,
                                                    &npub_seller,
                                                    None,
                                                    btc_amount_buyer,
                                                    btc_amount_seller,
                                                    funding_txid,
                                                    fee,
                                                    network,
                                                )
                                                .unwrap();
                                            consensus::serialize(&escrow_tx).as_hex().to_string()
                                        };
                                        #[cfg(debug_assertions)]
                                        trace!(% resolved_escrow_transaction, "Derived escrow transaction");
                                        escrow_transaction.set(resolved_escrow_transaction);
                                    },
                                    text: "Generate Transaction",
                                }
                                ContinueButton {
                                    to: Route::Sign {},
                                    text: "Continue to Sign",
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
