//! Satoshi Escrow Dixous App

use dioxus::prelude::*;

#[cfg(debug_assertions)]
use dioxus::logger::{
    self,
    tracing::{Level, info},
};

pub mod components;
pub mod error;
pub mod esplora;
pub mod scripts;
pub mod sign;
pub mod tx;
pub mod util;

use components::{Home, Navbar};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    #[cfg(debug_assertions)]
    {
        // init logger for Dioxus
        logger::init(Level::INFO).expect("failed to init logger");
    }
    // launch the web app
    #[cfg(debug_assertions)]
    info!("Launching Satoshi Escrow app");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link {
            rel: "stylesheet",
            href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css",
            integrity: "sha384-iw3OoTErCYJJB9mCa8LNS2hbsQ7M3C0EpIsO/H5+EGAkPGc6rk+V8i04oW/K5xq0",
            crossorigin: "anonymous",
        }
        Router::<Route> {}
    }
}
