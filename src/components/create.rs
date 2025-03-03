//! Create escrow transaction component.

use bitcoin::{Amount, Txid, consensus, hex::DisplayHex};
use dioxus::prelude::*;

#[cfg(debug_assertions)]
use dioxus::logger::tracing::{info, trace};

use crate::{
    NETWORK, Route,
    scripts::escrow_address,
    tx::escrow_tx,
    util::{
        P2TR_TX_WEIGHT_FUNDING, days_to_blocks, hours_to_blocks, npub_to_address, parse_network,
        parse_npub,
    },
};

use super::{
    BitcoinInput, ContinueButton, CopyButton, FeeRateInput, Footer, NetworkInput, NpubInput,
    NpubInputDerivedAddress, PrimaryButton,
};

/// Create escrow transaction component.
#[component]
pub(crate) fn Create() -> Element {
    let npub_buyer = use_signal(String::new);
    let npub_seller = use_signal(String::new);
    let npub_arbitrator = use_signal(String::new);
    let amount_buyer = use_signal(String::new);
    let amount_seller = use_signal(String::new);
    let fee_rate = use_signal(|| "1".to_string());
    let mut timelock_days = use_signal(String::new);
    let mut timelock_hours = use_signal(String::new);
    let mut funding_txid = use_signal(String::new);
    let mut escrow_address_str = use_signal(String::new);
    let mut escrow_transaction = use_signal(String::new);
    let mut derived_address_buyer = use_signal(String::new);
    let mut derived_address_seller = use_signal(String::new);
    rsx! {
        main { class: "max-w-7xl mx-auto py-6 sm:px-6 lg:px-8",
            div { class: "px-4 py-6 sm:px-0",
                h1 { class: "text-2xl font-bold text-gray-900 mb-6", "Create Escrow" }

                div { class: "bg-white shadow overflow-hidden sm:rounded-lg",
                    div { class: "px-4 py-5 sm:p-6",
                        div { class: "space-y-6",
                            div { class: "grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6",

                                NpubInputDerivedAddress {
                                    id: "npub_buyer",
                                    label: "Buyer Nostr Public Key (npub)",
                                    update_var: npub_buyer,
                                    update_address: derived_address_buyer,
                                }

                                NpubInputDerivedAddress {
                                    id: "npub_seller",
                                    label: "Seller Nostr Public Key (npub)",
                                    update_var: npub_seller,
                                    update_address: derived_address_seller,
                                }

                                BitcoinInput {
                                    id: "amount_buyer",
                                    label: "Buyer Escrow Amount (BTC)",
                                    update_var: amount_buyer,
                                }

                                BitcoinInput {
                                    id: "amount_seller",
                                    label: "Seller Escrow Amount (BTC)",
                                    update_var: amount_seller,
                                }

                                FeeRateInput {
                                    id: "fee",
                                    label: "Fee rate (sats/vByte)",
                                    update_var: fee_rate,
                                }

                                NetworkInput { id: "network", label: "Bitcoin Network" }
                            }

                            div { class: "border-t border-gray-200 pt-6",
                                h3 { class: "text-lg font-medium text-gray-900",
                                    "Optional Dispute Resolution"
                                }

                                div { class: "mt-4 grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6",

                                    NpubInput {
                                        id: "npub_arbitrator",
                                        label: "Arbitrator Nostr Public Key (npub)",
                                        update_var: npub_arbitrator,
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
                                div { class: "grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6",
                                    div { class: "col-span-2",
                                        dt { class: "text-lg font-medium text-gray-900",
                                            "Deposit Address"
                                        }
                                        dd {
                                            id: "escrow-address",
                                            class: "mt-1 text-sm text-gray-900 break-all bg-gray-50 p-3 rounded",
                                            {
                                                if escrow_address_str.read().is_empty() {
                                                    "bc1p...".to_string()
                                                } else {
                                                    escrow_address_str.read().clone()
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            div { class: "border-t border-gray-200 pt-6",
                                div { class: "grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6",
                                    div { class: "col-span-3",
                                        dt { class: "text-lg font-medium text-gray-900",
                                            "Buyer Resolution Address"
                                        }
                                        dd {
                                            id: "buyer-address",
                                            class: "mt-1 text-sm text-gray-900 break-all bg-gray-50 p-3 rounded",
                                            {
                                                if derived_address_buyer.read().is_empty() {
                                                    "bc1p...".to_string()
                                                } else {
                                                    derived_address_buyer.read().clone()
                                                }
                                            }
                                        }
                                    }
                                    div { class: "col-span-3",
                                        dt { class: "text-lg font-medium text-gray-900",
                                            "Seller Resolution Address"
                                        }
                                        dd {
                                            id: "buyer-address",
                                            class: "mt-1 text-sm text-gray-900 break-all bg-gray-50 p-3 rounded",
                                            {
                                                if derived_address_seller.read().is_empty() {
                                                    "bc1p...".to_string()
                                                } else {
                                                    derived_address_seller.read().clone()
                                                }
                                            }
                                        }
                                    }
                                }
                            }


                            div { class: "mt-5 flex flex-col space-y-3 sm:flex-row sm:space-y-0 sm:space-x-3",
                                CopyButton {
                                    text: "Deposit Address",
                                    clipboard_text: escrow_address_str,
                                }
                                PrimaryButton {
                                    onclick: move |_| {
                                        #[cfg(debug_assertions)]
                                        trace!(
                                            % npub_buyer, % npub_seller, % amount_buyer, % amount_seller, % fee_rate, %
                                            NETWORK, % npub_arbitrator, % timelock_days, % timelock_hours,
                                            "Clicked Generate Address"
                                        );
                                        let npub_buyer = parse_npub(&npub_buyer.read()).unwrap();
                                        let npub_seller = parse_npub(&npub_seller.read()).unwrap();
                                        let network = parse_network(&NETWORK.read()).unwrap();
                                        *derived_address_buyer.write() = npub_to_address(&npub_buyer, network)
                                            .unwrap()
                                            .to_string();
                                        *derived_address_seller.write() = npub_to_address(&npub_seller, network)
                                            .unwrap()
                                            .to_string();
                                        let resolved_escrow_address = if !npub_arbitrator.read().is_empty() {
                                            #[cfg(debug_assertions)]
                                            trace!("dispute escrow address");
                                            let npub_arbitrator = parse_npub(&npub_arbitrator.read()).unwrap();
                                            let timelock_hours = hours_to_blocks(
                                                timelock_hours.read().parse::<u32>().unwrap(),
                                            );
                                            let timelock_days = days_to_blocks(
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
                                        info!(% resolved_escrow_address, "Derived escrow address");
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
                                    class: "block text-md font-medium text-gray-700",
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
                                            placeholder: "Transaction data will appear here...",
                                            value: escrow_transaction,
                                        }
                                    }
                                }
                            }

                            div { class: "mt-5 flex flex-col space-y-3 sm:flex-row sm:space-y-0 sm:space-x-3",
                                CopyButton {
                                    text: "Transaction",
                                    clipboard_text: escrow_transaction,
                                }
                                PrimaryButton {
                                    onclick: move |_| {
                                        #[cfg(debug_assertions)]
                                        trace!(
                                            % npub_buyer, % npub_seller, % amount_buyer, % amount_seller, % fee_rate, %
                                            NETWORK, % npub_arbitrator, % timelock_days, % timelock_hours,
                                            "Clicked Generate Transaction"
                                        );
                                        let npub_buyer = parse_npub(&npub_buyer.read()).unwrap();
                                        let npub_seller = parse_npub(&npub_seller.read()).unwrap();
                                        let btc_amount_buyer = Amount::from_btc(
                                                amount_buyer.read().parse::<f64>().unwrap(),
                                            )
                                            .unwrap();
                                        let btc_amount_seller = Amount::from_btc(
                                                amount_seller.read().parse::<f64>().unwrap(),
                                            )
                                            .unwrap();
                                        let fee_rate = fee_rate.read().parse::<u64>().unwrap();
                                        let fee = Amount::from_sat(fee_rate * P2TR_TX_WEIGHT_FUNDING);
                                        let network = parse_network(&NETWORK.read()).unwrap();
                                        let funding_txid = funding_txid.read().parse::<Txid>().unwrap();
                                        let resolved_escrow_transaction = if !npub_arbitrator.read().is_empty() {
                                            #[cfg(debug_assertions)]
                                            trace!("dispute escrow address");
                                            let timelock_hours = hours_to_blocks(
                                                timelock_hours.read().parse::<u32>().unwrap(),
                                            );
                                            let timelock_days = days_to_blocks(
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
                                        info!(% resolved_escrow_transaction, "Derived escrow transaction");
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
