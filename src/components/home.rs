use dioxus::prelude::*;

/// Home page
#[component]
pub(crate) fn Home() -> Element {
    rsx! {
        div {
            "Hello, world!"
        }
    }
}
