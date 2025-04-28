//! Satoshi Escrow Dixous App

#![allow(non_snake_case)]

use dioxus::prelude::*;

#[cfg(debug_assertions)]
use dioxus::logger::{
    self,
    tracing::{Level, info},
};

pub(crate) mod components;
pub(crate) mod error;
pub(crate) mod esplora;
pub(crate) mod scripts;
pub(crate) mod sign;
pub(crate) mod tx;
pub(crate) mod util;
pub(crate) mod validation;

use components::{Broadcast, Combine, Create, Home, Navbar, Settings, Sign, Spend};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/create")]
        Create {},
        #[route("/sign")]
        Sign {},
        #[route("/combine")]
        Combine {},
        #[route("/broadcast")]
        Broadcast {},
        #[route("/spend")]
        Spend {},
        #[route("/settings")]
        Settings {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const LOGO: Asset = asset!("/assets/logo.svg");

/// The default network
static NETWORK: GlobalSignal<String> = Global::new(|| "Mainnet".to_string());

/// The default esplora endpoint
static ESPLORA_ENDPOINT: GlobalSignal<String> =
    Global::new(|| "https://mempool.space/api".to_string());

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
        Router::<Route> {}
    }
}
