//! Combine escrow signatures component.

use bitcoin::{Transaction, consensus, hex::DisplayHex};
use dioxus::prelude::*;

#[cfg(debug_assertions)]
use dioxus::logger::tracing::{info, trace};
use secp256k1::schnorr;

use crate::{
    Route,
    scripts::{escrow_scripts, escrow_spend_info},
    sign::combine_signatures,
    util::{days_to_blocks, hours_to_blocks, parse_escrow_type, parse_npub},
};

use super::{
    ContinueButton, CopyButton, EscrowTypeInput, Footer, NpubInput, PrimaryButton, SignatureInput,
    TimelockInput, TransactionInput, TransactionOutput,
};

/// Combine escrow transaction component.
#[component]
pub(crate) fn Combine() -> Element {
    let unsigned_tx = use_signal(String::new);
    let mut signed_tx_str = use_signal(String::new);
    let escrow_type = use_signal(String::new);
    let npub_buyer = use_signal(String::new);
    let npub_seller = use_signal(String::new);
    let signature_1 = use_signal(String::new);
    let signature_2 = use_signal(String::new);
    let npub_arbitrator = use_signal(String::new);
    let timelock_days = use_signal(String::new);
    let timelock_hours = use_signal(String::new);
    let signature_arbitrator = use_signal(String::new);
    rsx! {
        main { class: "max-w-7xl mx-auto py-6 sm:px-6 lg:px-8",
            div { class: "px-4 py-6 sm:px-0",
                h1 { class: "text-2xl font-bold text-gray-900 mb-6", "Combine Signatures" }

                div { class: "bg-white shadow overflow-hidden sm:rounded-lg",
                    div { class: "px-4 py-5 sm:p-6",
                        div { class: "space-y-6",

                            TransactionInput {
                                update_var: unsigned_tx,
                                label: "Unsigned Transaction",
                                id: "unsigned-tx",
                            }


                            div { class: "grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6",

                                NpubInput {
                                    id: "npub_1",
                                    label: "First Nostr Public Key (npub)",
                                    update_var: npub_buyer,
                                }

                                NpubInput {
                                    id: "npub_2",
                                    label: "Second Nostr Public Key (npub)",
                                    update_var: npub_seller,
                                }

                                SignatureInput {
                                    update_var: signature_1,
                                    label: "First Signature",
                                    id: "signature1",
                                }

                                SignatureInput {
                                    update_var: signature_2,
                                    label: "Second Signature",
                                    id: "signature2",
                                }

                                EscrowTypeInput { update_var: escrow_type }
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


                                    SignatureInput {
                                        update_var: signature_arbitrator,
                                        label: "Arbitrator Signature",
                                        id: "signaturearb",
                                    }
                                }
                            }

                            div { class: "pt-5",
                                div { class: "flex justify-end",
                                    PrimaryButton {
                                        onclick: move |_| {
                                            #[cfg(debug_assertions)]
                                            trace!(
                                                % npub_buyer, % npub_seller, % signature_1, % signature_2, % npub_arbitrator,
                                                % signature_arbitrator, % timelock_days, % timelock_hours, % escrow_type,
                                                "Clicked Combine Signatures"
                                            );
                                            let npub_buyer = parse_npub(&npub_buyer.read()).unwrap();
                                            let npub_seller = parse_npub(&npub_seller.read()).unwrap();
                                            let escrow_type = parse_escrow_type(&escrow_type.read()).unwrap();
                                            let unsigned_tx: Transaction = consensus::encode::deserialize_hex(
                                                    &unsigned_tx.read(),
                                                )
                                                .unwrap();
                                            let signatures: Vec<schnorr::Signature> = vec![
                                                signature_1.read(),
                                                signature_2.read(),
                                                signature_arbitrator.read(),
                                            ]
                                                .into_iter()
                                                .filter(|s| !s.is_empty())
                                                .map(|s| s.parse::<schnorr::Signature>().unwrap())
                                                .collect();
                                            let signed_tx = if !npub_arbitrator.read().is_empty() {
                                                #[cfg(debug_assertions)]
                                                trace!("dispute escrow combine signatures");
                                                let npub_arbitrator = parse_npub(&npub_arbitrator.read()).unwrap();
                                                let timelock_hours = hours_to_blocks(
                                                    timelock_hours.read().parse::<u32>().unwrap(),
                                                );
                                                let timelock_days = days_to_blocks(
                                                    timelock_days.read().parse::<u32>().unwrap(),
                                                );
                                                let timelock_duration = timelock_days + timelock_hours;
                                                let locking_script = escrow_scripts(
                                                        &npub_buyer,
                                                        &npub_seller,
                                                        Some(&npub_arbitrator),
                                                        Some(timelock_duration),
                                                        escrow_type,
                                                    )
                                                    .unwrap();
                                                let taproot_spend_info = escrow_spend_info(
                                                        &npub_buyer,
                                                        &npub_seller,
                                                        Some(&npub_arbitrator),
                                                        Some(timelock_duration),
                                                    )
                                                    .unwrap();
                                                let signed_tx = combine_signatures(
                                                    unsigned_tx,
                                                    0,
                                                    signatures.iter().collect::<Vec<&schnorr::Signature>>(),
                                                    &locking_script,
                                                    &taproot_spend_info,
                                                );
                                                consensus::serialize(&signed_tx).as_hex().to_string()
                                            } else {
                                                #[cfg(debug_assertions)]
                                                trace!("collaborative escrow combine signatures");
                                                let locking_script = escrow_scripts(
                                                        &npub_buyer,
                                                        &npub_seller,
                                                        None,
                                                        None,
                                                        escrow_type,
                                                    )
                                                    .unwrap();
                                                let taproot_spend_info = escrow_spend_info(
                                                        &npub_buyer,
                                                        &npub_seller,
                                                        None,
                                                        None,
                                                    )
                                                    .unwrap();
                                                let signed_tx = combine_signatures(
                                                    unsigned_tx,
                                                    0,
                                                    signatures.iter().collect::<Vec<&schnorr::Signature>>(),
                                                    &locking_script,
                                                    &taproot_spend_info,
                                                );
                                                consensus::serialize(&signed_tx).as_hex().to_string()
                                            };
                                            #[cfg(debug_assertions)]
                                            info!(% signed_tx, "Combined signatures into a signed transaction");
                                            signed_tx_str.set(signed_tx);
                                        },
                                        text: "Combine Signatures",
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

                        TransactionOutput {
                            update_var: signed_tx_str,
                            label: "",
                            id: "signed-tx",
                            placeholder: "Signed transaction will appear here...",
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
