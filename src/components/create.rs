//! Create escrow transaction component.

use bitcoin::{Amount, Txid, consensus, hex::DisplayHex};
use dioxus::prelude::*;

#[cfg(debug_assertions)]
use dioxus::logger::tracing::{info, trace};

use crate::{
    ESPLORA_ENDPOINT, NETWORK, Route,
    esplora::{FeeEstimate, create_client, get_fee_estimates},
    scripts::escrow_address,
    tx::escrow_tx,
    util::{
        P2TR_TX_VBYTE_C, days_to_blocks, hours_to_blocks, npub_to_address, parse_network,
        parse_npub,
    },
};

use super::{
    BitcoinInput, ContinueButton, CopyButton, DerivedAddressOutput, FeeRateSelector, Footer,
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
    let mut fee_rate = use_signal(String::new);
    let fee_estimates = use_signal(|| Option::<FeeEstimate>::None);
    let timelock_days = use_signal(String::new);
    let timelock_hours = use_signal(String::new);
    let funding_txid = use_signal(String::new);
    let mut escrow_address_str = use_signal(String::new);
    let mut escrow_transaction = use_signal(String::new);
    let mut derived_address_buyer = use_signal(String::new);
    let mut derived_address_seller = use_signal(String::new);

    let mut npub_buyer_error = use_signal(|| None);
    let mut npub_seller_error = use_signal(|| None);
    let npub_arbitrator_error = use_signal(|| None);
    let mut amount_buyer_error = use_signal(|| None);
    let mut amount_seller_error = use_signal(|| None);
    let mut fee_rate_error = use_signal(|| None);
    let mut timelock_days_error = use_signal(|| Option::<String>::None);
    let mut timelock_hours_error = use_signal(|| Option::<String>::None);
    let mut funding_txid_error = use_signal(|| Option::<String>::None);

    let has_address_form_errors = move || {
        npub_buyer_error.read().is_some()
            || npub_seller_error.read().is_some()
            || amount_buyer_error.read().is_some()
            || amount_seller_error.read().is_some()
            || fee_rate_error.read().is_some()
            || npub_arbitrator_error.read().is_some()
            || timelock_days_error.read().is_some()
            || timelock_hours_error.read().is_some()
    };

    let mut validate_address_form = move || {
        if npub_buyer.read().is_empty() {
            npub_buyer_error.set(Some("Buyer npub is required.".to_string()));
        }

        if npub_seller.read().is_empty() {
            npub_seller_error.set(Some("Seller npub is required.".to_string()));
        }

        if amount_buyer.read().is_empty() {
            amount_buyer_error.set(Some("Buyer amount is required.".to_string()));
        }

        if amount_seller.read().is_empty() {
            amount_seller_error.set(Some("Seller amount is required.".to_string()));
        }

        if fee_rate.read().is_empty() {
            fee_rate_error.set(Some("Fee rate is required.".to_string()));
        }

        let arbitrator_filled = !npub_arbitrator.read().is_empty();

        if arbitrator_filled {
            if timelock_days.read().is_empty() {
                timelock_days_error.set(Some("Timelock (days) is required.".to_string()));
            }
            if timelock_hours.read().is_empty() {
                timelock_hours_error.set(Some("Timelock (hours) is required.".to_string()));
            }
        }
    };

    let has_transaction_form_errors = move || funding_txid_error.read().is_some();

    let mut validate_transaction_form = move || {
        if funding_txid.read().is_empty() {
            funding_txid_error.set(Some("Transaction ID is required.".to_string()));
        }
    };

    use_effect(move || {
        to_owned![fee_estimates];

        spawn(async move {
            let esplora_client = create_client(&ESPLORA_ENDPOINT.read()).unwrap();
            match get_fee_estimates(&esplora_client).await {
                Ok(estimates) => {
                    #[cfg(debug_assertions)]
                    trace!("Fee estimates fetched successfully: {:?}", estimates);
                    fee_estimates.set(Some(estimates));
                }
                Err(e) => {
                    #[cfg(debug_assertions)]
                    trace!("Error fetching fee estimates: {}", e);
                    // Fall back to 3 sat/vB
                    fee_rate.set("3".to_string());
                }
            }
        });
    });

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
                                    error: npub_buyer_error,
                                }

                                NpubInputDerivedAddress {
                                    id: "npub_seller",
                                    label: "Seller Nostr Public Key (npub)",
                                    update_var: npub_seller,
                                    update_address: derived_address_seller,
                                    col_span: 3,
                                    error: npub_seller_error,
                                }

                                BitcoinInput {
                                    id: "amount_buyer",
                                    label: "Buyer Escrow Amount (BTC)",
                                    update_var: amount_buyer,
                                    error: amount_buyer_error,
                                }

                                BitcoinInput {
                                    id: "amount_seller",
                                    label: "Seller Escrow Amount (BTC)",
                                    update_var: amount_seller,
                                    error: amount_seller_error,
                                }

                                FeeRateSelector {
                                    id: "fee",
                                    label_input: "Fee rate (sats/vByte)",
                                    label_dropdown: "Target Blocks",
                                    update_var: fee_rate,
                                    fee_estimates,
                                    error: fee_rate_error,
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
                                        error: npub_arbitrator_error,
                                    }

                                    TimelockInput {
                                        update_day_var: timelock_days,
                                        update_hour_var: timelock_hours,
                                        day_error: timelock_days_error,
                                        hour_error: timelock_hours_error,
                                        required: !npub_arbitrator.read().is_empty()
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
                                        validate_address_form();

                                        if has_address_form_errors() {
                                            #[cfg(debug_assertions)]
                                            trace!("Form has validation errors, cannot generate address");
                                            return;
                                        }

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
                                error: funding_txid_error,
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
                                        validate_transaction_form();

                                        if has_transaction_form_errors() {
                                            #[cfg(debug_assertions)]
                                            trace!("Form has validation errors, cannot generate transaction");
                                            return;
                                        }

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
