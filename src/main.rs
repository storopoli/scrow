//! Satoshi Escrow Dixous App

#![allow(non_snake_case)]

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

use components::{Broadcast, Combine, Create, Home, Navbar, Settings, Sign};

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
        #[route("/settings")]
        Settings {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const LOGO: Asset = asset!("/assets/logo.svg");

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
