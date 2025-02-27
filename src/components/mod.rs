//! Dioxus Components.

// Sanity checks
use dioxus::prelude::*;

#[component]
pub fn Button(text: String) -> Element {
    rsx! {
        button {
            class: "btn btn-primary",
            {text}
        }
    }
}
