//! Home page component.

use dioxus::prelude::*;

use crate::{Route, components::Footer};

/// Home page component.
#[component]
pub(crate) fn Home() -> Element {
    rsx! {
        main { class: "max-w-7xl mx-auto py-6 sm:px-6 lg:px-8",
            div { class: "px-4 py-6 sm:px-0",
                div { class: "prose max-w-none",
                    h1 { class: "text-4xl font-bold text-gray-900 mb-4", "Satoshi Escrow" }
                    p { class: "text-xl text-gray-600 mb-8",
                        "Satoshi Escrow: A Bitcoin non-custodial peer-to-peer dispute resolution using only Nostr keys."
                    }
                    div { class: "mt-8",
                        h2 { class: "text-2xl font-semibold text-gray-900 mb-4", "Motivation" }
                        p { class: "text-gray-600 mb-4",
                            "When a Buyer wants to purchase something for a certain amount of BTC from a Seller but doesn't trust them,
                            they can use our 2-of-2 multisig escrow address. The Buyer and Seller both lock their BTC in a multisig address.
                            Both parties only need their respective Nostr secret
                            keys (nsec) and each other's Nostr public keys (npub)."
                        }
                        div { class: "bg-white shadow-sm rounded-lg p-6 mb-8",
                            h3 { class: "text-lg font-semibold text-gray-900 mb-4",
                                "How it works"
                            }
                            ol { class: "list-decimal list-inside space-y-2 text-gray-600",
                                li {
                                    "If the trade is successful:"
                                    ul { class: "pl-6 mt-2 list-disc",
                                        li { "Both parties sign to release the funds" }
                                        li { "Buyer receives his BTC back" }
                                        li { "Seller receives his BTC back" }
                                    }
                                }
                                li { class: "pt-2",
                                    "If there's a dispute:"
                                    ul { class: "pl-6 mt-2 list-disc",
                                        li { "Parties can choose a trusted third party (arbitrator)" }
                                        li {
                                            "Arbitrator can help resolve the dispute after a timelock period"
                                        }
                                        li {
                                            "Resolution requires 2-of-3 signatures (Buyer/Seller + Arbitrator)"
                                        }
                                    }
                                }
                            }
                        }
                        h2 { class: "text-2xl font-semibold text-gray-900 mb-4",
                            "Technical Implementation"
                        }
                        p { class: "text-gray-600 mb-4",
                            "We use Pay-to-Taproot (P2TR) multisig script path spends with a verified unknown
                            discrete-log unspendable internal key. The system supports two resolution paths:"
                        }
                        div { class: "grid md:grid-cols-2 gap-6 mb-8",
                            div { class: "bg-white shadow-sm rounded-lg p-6",
                                h3 { class: "text-lg font-semibold text-gray-900 mb-3",
                                    "Collaborative Resolution"
                                }
                                p { class: "text-gray-600",
                                    "2-of-2 multisig between Buyer and Seller without timelocks."
                                }
                            }
                            div { class: "bg-white shadow-sm rounded-lg p-6",
                                h3 { class: "text-lg font-semibold text-gray-900 mb-3",
                                    "Dispute Resolution"
                                }
                                p { class: "text-gray-600",
                                    "2-of-3 multisig between either party and the arbitrator with timelock."
                                }
                            }
                        }
                        div { class: "bg-yellow-50 border-l-4 border-yellow-400 p-4 mb-8",
                            div { class: "flex",
                                div { class: "flex-shrink-0",
                                    i { class: "text-yellow-400",
                                        svg {
                                            xmlns: "http://www.w3.org/2000/svg",
                                            width: "24",
                                            height: "24",
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            "stroke-width": "2",
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            class: "lucide lucide-circle-alert",

                                            circle { cx: "12", cy: "12", r: "10" }
                                            line {
                                                x1: "12",
                                                x2: "12",
                                                y1: "8",
                                                y2: "12",
                                            }
                                            line {
                                                x1: "12",
                                                x2: "12.01",
                                                y1: "16",
                                                y2: "16",
                                            }
                                        }
                                    }
                                }
                                div { class: "ml-3",
                                    h3 { class: "text-sm font-medium text-yellow-800",
                                        "Important Security Notice"
                                    }
                                    div { class: "mt-2 text-sm text-yellow-700",
                                        p {
                                            "This application can be used offline on an air-gapped computer for maximum security.
                                            All transactions can be generated, and signed through the webpage offline."
                                        }
                                    }
                                }
                            }
                        }
                        h2 { class: "text-2xl font-semibold text-gray-900 mb-4", "Getting Started" }
                        div { class: "grid md:grid-cols-5 gap-6",
                            Link {
                                to: Route::Create {},
                                class: "block bg-white shadow-sm rounded-lg p-6 hover:shadow-md transition-shadow",
                                h3 { class: "text-lg font-semibold text-gray-900 mb-2",
                                    "1. Create Escrow"
                                }
                                p { class: "text-gray-600",
                                    "Set up a new escrow address using npubs and specify amounts."
                                }
                            }
                            Link {
                                to: Route::Sign {},
                                class: "block bg-white shadow-sm rounded-lg p-6 hover:shadow-md transition-shadow",
                                h3 { class: "text-lg font-semibold text-gray-900 mb-2",
                                    "2. Sign Transaction"
                                }
                                p { class: "text-gray-600",
                                    "Sign the transaction using your nsec key."
                                }
                            }
                            Link {
                                to: Route::Combine {},
                                class: "block bg-white shadow-sm rounded-lg p-6 hover:shadow-md transition-shadow",
                                h3 { class: "text-lg font-semibold text-gray-900 mb-2",
                                    "3. Combine Signatures"
                                }
                                p { class: "text-gray-600",
                                    "Combine the signatures into a signed transaction."
                                }
                            }
                            Link {
                                to: Route::Home {}, // TODO: Replace with Route::Broadcast {} when available
                                class: "block bg-white shadow-sm rounded-lg p-6 hover:shadow-md transition-shadow",
                                h3 { class: "text-lg font-semibold text-gray-900 mb-2",
                                    "4. Broadcast"
                                }
                                p { class: "text-gray-600",
                                    "Broadcast the signed transaction to the Bitcoin network."
                                }
                            }
                            Link {
                                to: Route::Home {}, // TODO: Replace with Route::Spend {} when available
                                class: "block bg-white shadow-sm rounded-lg p-6 hover:shadow-md transition-shadow",
                                h3 { class: "text-lg font-semibold text-gray-900 mb-2",
                                    "5. Spend"
                                }
                                p { class: "text-gray-600",
                                    "Spend from the resolution address derived from your npub using your nsec."
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
