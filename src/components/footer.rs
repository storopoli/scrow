//! Footer component.

use dioxus::prelude::*;

/// Footer component.
#[component]
pub(crate) fn Footer() -> Element {
    rsx! {
        footer { class: "bg-white mt-12 border-t border-gray-200",
            div { class: "max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8",
                p { class: "text-center text-gray-500 text-sm",
                    "Satoshi Escrow - Open Source Bitcoin Dispute Resolution"
                }
            }
        }
    }
}
