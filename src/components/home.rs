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
                    div { class: "mt-5",
                        a {
                            href: "https://github.com/storopoli/scrow",
                            target: "_blank",
                            class: "inline-flex items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
                            // GitHub icon SVG
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                width: "16",
                                height: "16",
                                view_box: "0 0 24 24",
                                fill: "currentColor",
                                class: "mr-2",
                                path { d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" }
                            }
                            "View on GitHub"
                        }
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
                                to: Route::Broadcast {},
                                class: "block bg-white shadow-sm rounded-lg p-6 hover:shadow-md transition-shadow",
                                h3 { class: "text-lg font-semibold text-gray-900 mb-2",
                                    "4. Broadcast"
                                }
                                p { class: "text-gray-600",
                                    "Broadcast the signed transaction to the Bitcoin network."
                                }
                            }
                            Link {
                                to: Route::Spend {},
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
