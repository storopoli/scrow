//! Dioxus Components.

pub(crate) mod navbar;
pub(crate) use navbar::Navbar;

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
