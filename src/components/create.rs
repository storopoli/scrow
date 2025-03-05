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
        P2TR_TX_VBYTE_C, days_to_blocks, hours_to_blocks, npub_to_address, parse_network,
        parse_npub,
    },
};

use super::{
    BitcoinInput, ContinueButton, CopyButton, DerivedAddressOutput, FeeRateInput, Footer,
    NetworkInput, NpubInput, NpubInputDerivedAddress, PrimaryButton, TimelockInput,
    TransactionOutput, TxidInput,
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
    let timelock_days = use_signal(String::new);
    let timelock_hours = use_signal(String::new);
    let funding_txid = use_signal(String::new);
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
                                    col_span: 3,
                                }

                                NpubInputDerivedAddress {
                                    id: "npub_seller",
                                    label: "Seller Nostr Public Key (npub)",
                                    update_var: npub_seller,
                                    update_address: derived_address_seller,
                                    col_span: 3,
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

                                    TimelockInput {
                                        update_day_var: timelock_days,
                                        update_hour_var: timelock_hours,
                                    }
                                }
                            }

                            div { class: "border-t border-gray-200 pt-6",
                                div { class: "grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6",
                                    DerivedAddressOutput {
                                        update_var: escrow_address_str,
                                        label: "Deposit Address",
                                        id: "escrow-address",
                                        col_span: 3,
                                    }
                                }
                            }

                            div { class: "border-t border-gray-200 pt-6",
                                div { class: "grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6",
                                    DerivedAddressOutput {
                                        update_var: derived_address_buyer,
                                        label: "Buyer's Resolution Address",
                                        id: "buyer-address",
                                        col_span: 3,
                                    }

                                    DerivedAddressOutput {
                                        update_var: derived_address_seller,
                                        label: "Seller's Resolution Address",
                                        id: "seller-address",
                                        col_span: 3,
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
                            TxidInput {
                                update_var: funding_txid,
                                label: "Escrow funding Transaction ID",
                                warning: "Deposit a single transaction to the escrow address and inform the transaction ID.
                                This transaction will be used to fund the escrow address.
                                Note that it should be a coinjoin transaction between buyer and seller,
                                i.e. should have only one output: the escrow address with the whole total escrow amount.",
                            }
                        }


                        div { class: "mt-5 border-t border-gray-200 pt-5",
                            dl { class: "grid grid-cols-1 gap-x-4 gap-y-6 sm:grid-cols-2",
                                TransactionOutput {
                                    update_var: escrow_transaction,
                                    label: "Unsigned Escrow Resolution Transaction",
                                    id: "escrow-tx",
                                    placeholder: "Transaction data will appear here...",
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
                                        // We always assume the highest transaction weight
                                        let fee = Amount::from_sat(fee_rate * P2TR_TX_VBYTE_C);
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
