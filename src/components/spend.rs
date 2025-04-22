//! Spend from resolution address component.

use bitcoin::{Address, Amount, TxOut, Txid, consensus, hex::DisplayHex};
use dioxus::prelude::*;

#[cfg(debug_assertions)]
use dioxus::logger::tracing::trace;

use crate::{
    ESPLORA_ENDPOINT, NETWORK, Route,
    esplora::{FeeEstimate, create_client, get_fee_estimates},
    sign::sign_resolution_tx,
    tx::resolution_tx,
    util::{P2TR_TX_VBYTE_KEY_PATH, parse_network, parse_nsec},
};

use super::{
    AddressInput, BitcoinInput, ContinueButton, CopyButton, DerivedAddressOutput, FeeRateSelector,
    Footer, NetworkInput, NpubInputDerivedAddress, NsecInput, PrimaryButton, TransactionOutput,
    TxidInput, VoutInput,
};

/// Spend from resolution address component.
#[component]
pub(crate) fn Spend() -> Element {
    let npub = use_signal(String::new);
    let escrow_txid = use_signal(String::new);
    let destination_address = use_signal(String::new);
    let amount = use_signal(String::new);
    let mut fee_rate = use_signal(String::new);
    let fee_estimates = use_signal(|| Option::<FeeEstimate>::None);
    let vout = use_signal(|| "0".to_string());
    let derived_address = use_signal(String::new);
    let nsec = use_signal(String::new);
    let mut signed_tx_str = use_signal(String::new);

    let mut npub_error = use_signal(|| None);
    let mut escrow_txid_error = use_signal(|| None);
    let mut destination_address_error = use_signal(|| None);
    let mut amount_error = use_signal(|| None);
    let mut fee_rate_error = use_signal(|| None);
    let mut nsec_error = use_signal(|| None);

    let has_spend_form_errors = move || {
        npub_error.read().is_some()
            || escrow_txid_error.read().is_some()
            || destination_address_error.read().is_some()
            || amount_error.read().is_some()
            || fee_rate_error.read().is_some()
            || nsec_error.read().is_some()
    };

    let mut validate_spend_form = move || {
        if npub.read().is_empty() {
            npub_error.set(Some("Npub is required.".to_string()));
        }
        if escrow_txid.read().is_empty() {
            escrow_txid_error.set(Some("Transaction ID is required.".to_string()));
        }
        if destination_address.read().is_empty() {
            destination_address_error.set(Some("Destination address is required.".to_string()));
        }
        if amount.read().is_empty() {
            amount_error.set(Some("Amount is required.".to_string()));
        }
        if fee_rate.read().is_empty() {
            fee_rate_error.set(Some("Fee rate is required.".to_string()));
        }
        if nsec.read().is_empty() {
            nsec_error.set(Some("Nsec is required.".to_string()));
        }
    };

    use_effect(move || {
        to_owned![fee_estimates];

        spawn(async move {
            let esplora_client = create_client(&ESPLORA_ENDPOINT.read()).unwrap();
            match get_fee_estimates(&esplora_client).await {
                Ok(estimates) => {
                    #[cfg(debug_assertions)]
                    trace!(?estimates, "Fee estimates fetched successfully",);
                    fee_estimates.set(Some(estimates));
                }
                Err(e) => {
                    #[cfg(debug_assertions)]
                    trace!(%e, "Error fetching fee estimates: {}", e);
                    // Fall back to 1 sat/vB
                    fee_rate.set("1".to_string());
                }
            }
        });
    });

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
                                    col_span: 3,
                                    error: npub_error,
                                }

                                TxidInput {
                                    label: "Escrow Resolution Transaction ID",
                                    update_var: escrow_txid,
                                    warning: "",
                                    error: escrow_txid_error,
                                }

                                VoutInput {
                                    id: "escrow_vout",
                                    label: "Escrow Resolution Transaction Output Index",
                                    update_var: vout,
                                }

                                AddressInput { update_var: destination_address, error: destination_address_error }

                                BitcoinInput {
                                    id: "amount",
                                    label: "Total Locked Amount (BTC)",
                                    update_var: amount,
                                    error: amount_error,
                                }

                                FeeRateSelector {
                                    id: "fee",
                                    label_input: "Fee rate (sats/vByte)",
                                    label_dropdown: "Target Blocks",
                                    update_var: fee_rate,
                                    fee_estimates,
                                    error: fee_rate_error,
                                }

                                DerivedAddressOutput {
                                    update_var: derived_address,
                                    label: "Your Resolution Address",
                                    id: "derived-address",
                                    col_span: 3,
                                }

                                NsecInput { update_var: nsec, error: nsec_error, }
                            }

                            div { class: "pt-5",
                                div { class: "flex justify-end",
                                    PrimaryButton {
                                        onclick: move |_| {
                                            validate_spend_form();

                                            if has_spend_form_errors() {
                                                #[cfg(debug_assertions)]
                                                trace!("Form has validation errors, cannot sign transaction");
                                                return;
                                            }

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
                                            let vout = vout.read().parse::<u32>().unwrap();
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
                                            let fee = Amount::from_sat(fee_rate * P2TR_TX_VBYTE_KEY_PATH);
                                            let unsigned_tx = resolution_tx(
                                                btc_amount,
                                                escrow_txid,
                                                vout,
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
