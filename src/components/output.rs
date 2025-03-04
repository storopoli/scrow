//! Output Components.

use dioxus::prelude::*;

/// Transaction output component.
#[component]
pub(crate) fn TransactionOutput(
    mut update_var: Signal<String>,
    label: String,
    id: String,
    placeholder: String,
) -> Element {
    rsx! {
        div { class: "sm:col-span-6",
            label {
                r#for: id.as_str(),
                class: "block text-sm font-medium text-gray-500",
                {label}
            }
            div { class: "mt-1",
                textarea {
                    id: id.as_str(),
                    readonly: "true",
                    rows: "4",
                    class: "shadow-sm block w-full sm:text-sm border-gray-300 rounded-md p-2 border bg-gray-50",
                    placeholder: placeholder.as_str(),
                    value: update_var,
                }
            }
        }
    }
}

/// Signature output component.
#[component]
pub(crate) fn SignatureOutput(mut update_var: Signal<String>) -> Element {
    rsx! {
        div { class: "sm:col-span-6",
            div { class: "mt-1",
                textarea {
                    id: "signature",
                    readonly: "true",
                    rows: "4",
                    class: "shadow-sm block w-full sm:text-sm border-gray-300 rounded-md p-2 border bg-gray-50",
                    placeholder: "Signature will appear here...",
                    value: update_var,
                }
            }
        }
    }
}
