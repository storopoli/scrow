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

use super::{
    BitcoinInput, ContinueButton, CopyButton, EscrowTypeInput, Footer, NetworkInput, NpubInput,
    NsecInput, PrimaryButton, SignatureOutput, TimelockInput, TransactionInput, TxidInput,
};

/// Sign escrow transaction component.
#[component]
pub(crate) fn Sign() -> Element {
    let unsigned_tx = use_signal(String::new);
    let mut signature = use_signal(String::new);
    let escrow_type = use_signal(String::new);
    let npub_buyer = use_signal(String::new);
    let npub_seller = use_signal(String::new);
    let nsec = use_signal(String::new);
    let npub_arbitrator = use_signal(String::new);
    let amount_total = use_signal(String::new);
    let timelock_days = use_signal(String::new);
    let timelock_hours = use_signal(String::new);
    let funding_txid = use_signal(String::new);

    let mut npub_buyer_error = use_signal(|| None);
    let mut npub_seller_error = use_signal(|| None);
    let npub_arbitrator_error = use_signal(|| None);
    let mut amount_total_error = use_signal(|| None);
    let mut timelock_days_error = use_signal(|| None);
    let mut timelock_hours_error = use_signal(|| None);
    let mut funding_txid_error = use_signal(|| None);
    let mut unsigned_tx_error = use_signal(|| None);
    let mut nsec_error = use_signal(|| None);

    let has_sign_form_errors = move || {
        npub_buyer_error.read().is_some()
            || npub_seller_error.read().is_some()
            || npub_arbitrator_error.read().is_some()
            || amount_total_error.read().is_some()
            || timelock_days_error.read().is_some()
            || timelock_hours_error.read().is_some()
            || funding_txid_error.read().is_some()
            || unsigned_tx_error.read().is_some()
            || nsec_error.read().is_some()
    };

    let mut validate_sign_form = move || {
        if npub_buyer.read().is_empty() {
            npub_buyer_error.set(Some("Buyer npub is required.".to_string()));
        }

        if npub_seller.read().is_empty() {
            npub_seller_error.set(Some("Seller npub is required.".to_string()));
        }

        if amount_total.read().is_empty() {
            amount_total_error.set(Some("Amount is required.".to_string()));
        }

        if funding_txid.read().is_empty() {
            funding_txid_error.set(Some("Transaction ID is required.".to_string()));
        }

        if nsec.read().is_empty() {
            nsec_error.set(Some("Nsec is required.".to_string()));
        }

        if unsigned_tx.read().is_empty() {
            unsigned_tx_error.set(Some("Unsigned transaction is required.".to_string()));
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

    let var_name = rsx! {
        main { class: "max-w-7xl mx-auto py-6 sm:px-6 lg:px-8",
            div { class: "px-4 py-6 sm:px-0",
                h1 { class: "text-2xl font-bold text-gray-900 mb-6", "Sign Escrow" }

                div { class: "bg-white shadow overflow-hidden sm:rounded-lg",
                    div { class: "px-4 py-5 sm:p-6",
                        div { class: "space-y-6",

                            TransactionInput {
                                update_var: unsigned_tx,
                                label: "Unsigned Transaction",
                                id: "unsigned-tx",
                                error: unsigned_tx_error,
                            }

                            div { class: "grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6",
                                NetworkInput { id: "network", label: "Bitcoin Network" }

                                EscrowTypeInput { update_var: escrow_type }

                                NpubInput {
                                    id: "npub_buyer",
                                    label: "Buyer Nostr Public Key (npub)",
                                    update_var: npub_buyer,
                                    error: npub_buyer_error,
                                }

                                NpubInput {
                                    id: "npub_seller",
                                    label: "Seller Nostr Public Key (npub)",
                                    update_var: npub_seller,
                                    error: npub_seller_error,
                                }

                                TxidInput {
                                    label: "Escrow funding Transaction ID",
                                    update_var: funding_txid,
                                    warning: "",
                                    error: funding_txid_error,
                                }


                                BitcoinInput {
                                    id: "amount_total",
                                    label: "Total Locked Escrow Amount (BTC)",
                                    update_var: amount_total,
                                    error: amount_total_error,
                                }

                                NsecInput { update_var: nsec, error: nsec_error }
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
                                        error: npub_arbitrator_error,
                                    }

                                    TimelockInput {
                                        update_day_var: timelock_days,
                                        update_hour_var: timelock_hours,
                                        day_error: timelock_days_error,
                                        hour_error: timelock_hours_error,
                                        required: !npub_arbitrator.read().is_empty(),
                                    }
                                }
                            }

                            div { class: "pt-5",
                                div { class: "flex justify-end",
                                    PrimaryButton {
                                        onclick: move |_| {
                                            validate_sign_form();
                                            if has_sign_form_errors() {
                                                #[cfg(debug_assertions)]
                                                trace!("Form has validation errors, cannot sign transaction");
                                                return;
                                            }
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
                                            let unsigned_tx: Transaction = consensus::encode::deserialize_hex(
                                                    &unsigned_tx.read(),
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

                        SignatureOutput { update_var: signature }

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
