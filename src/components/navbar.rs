use dioxus::prelude::*;

use crate::Route;

/// Shared navbar component.
#[component]
pub(crate) fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
        }
        Outlet::<Route> {}
    }
}
