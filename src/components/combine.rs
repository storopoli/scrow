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

use super::{ContinueButton, CopyButton, Footer, PrimaryButton};

/// Combine escrow transaction component.
#[component]
pub(crate) fn Combine() -> Element {
    let mut unsigned_tx = use_signal(String::new);
    let mut signed_tx_str = use_signal(String::new);
    let mut escrow_type = use_signal(String::new);
    let mut npub_buyer = use_signal(String::new);
    let mut npub_seller = use_signal(String::new);
    let mut signature_1 = use_signal(String::new);
    let mut signature_2 = use_signal(String::new);
    let mut npub_arbitrator = use_signal(String::new);
    let mut timelock_days = use_signal(String::new);
    let mut timelock_hours = use_signal(String::new);
    let mut signature_arbitrator = use_signal(String::new);
    rsx! {
        main { class: "max-w-7xl mx-auto py-6 sm:px-6 lg:px-8",
            div { class: "px-4 py-6 sm:px-0",
                h1 { class: "text-2xl font-bold text-gray-900 mb-6", "Combine Signatures" }

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
                                        r#for: "npub1",
                                        class: "block text-sm font-medium text-gray-700",
                                        "First Party Nostr Public Key (npub)"
                                    }
                                    div { class: "mt-1",
                                        input {
                                            r#type: "text",
                                            name: "npub1",
                                            id: "npub1",
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
                                        r#for: "npub2",
                                        class: "block text-sm font-medium text-gray-700",
                                        "Second Party Nostr Public Key (npub)"
                                    }
                                    div { class: "mt-1",
                                        input {
                                            r#type: "text",
                                            name: "npub2",
                                            id: "npub2",
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
                                        r#for: "signature1",
                                        class: "block text-sm font-medium text-gray-700",
                                        "First Signature"
                                    }
                                    div { class: "mt-1",
                                        textarea {
                                            id: "signature1",
                                            name: "signature1",
                                            rows: "2",
                                            class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                            placeholder: "Paste signature here...",
                                            oninput: move |event| {
                                                #[cfg(debug_assertions)]
                                                trace!(% signature_1, event_value =% event.value(), "Set signature 1");
                                                signature_1.set(event.value());
                                            },
                                        }
                                    }
                                }

                                div { class: "sm:col-span-3",
                                    label {
                                        r#for: "signature2",
                                        class: "block text-sm font-medium text-gray-700",
                                        "Second Signature"
                                    }
                                    div { class: "mt-1",
                                        textarea {
                                            id: "signature2",
                                            name: "signature2",
                                            rows: "2",
                                            class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                            placeholder: "Paste signature here...",
                                            oninput: move |event| {
                                                #[cfg(debug_assertions)]
                                                trace!(% signature_2, event_value =% event.value(), "Set signature 2");
                                                signature_2.set(event.value());
                                            },
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

                                    div { class: "sm:col-span-3",
                                        label {
                                            r#for: "signaturearb",
                                            class: "block text-sm font-medium text-gray-700",
                                            "Arbitrator Signature"
                                        }
                                        div { class: "mt-1",
                                            textarea {
                                                id: "signaturearb",
                                                name: "signaturearb",
                                                rows: "2",
                                                class: "shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md p-2 border",
                                                placeholder: "Paste signature here...",
                                                oninput: move |event| {
                                                    #[cfg(debug_assertions)]
                                                    trace!(
                                                        % signature_arbitrator, event_value =% event.value(),
                                                        "Set signature arbitrator"
                                                    );
                                                    signature_arbitrator.set(event.value());
                                                },
                                            }
                                        }
                                    }
                                }
                            }

                            div { class: "pt-5",
                                div { class: "flex justify-end",
                                    // TODO: Use PrimaryButton with a custom onclick
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
                                            let unsigned_tx: Transaction = consensus::deserialize(
                                                    unsigned_tx.read().as_bytes(),
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

                // Result Section (would be shown after form submission)
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
                                        placeholder: "Signed transaction will appear here...",
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
