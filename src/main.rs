//! Satoshi Escrow Dixous App

use bitcoin::{Address, Network};
use dioxus::prelude::*;
use esplora::{create_client, get_funding_txid};
use scripts::UNSPENDABLE_PUBLIC_KEY;

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

use components::Button;
use components::Navbar;

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
        Router::<Route> {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    let unspendable_pk = *UNSPENDABLE_PUBLIC_KEY;
    let txid = use_resource(move || async move {
        let client =
            create_client("https://mempool.space/testnet4/api/").expect("could not create client");
        let address = "tb1q8tpam3snku72xz9sx3rxerrcqmqd2ljdq95k8j"
            .parse::<Address<_>>()
            .unwrap()
            .require_network(Network::Testnet)
            .unwrap();
        get_funding_txid(&client, &address)
            .await
            .unwrap()
            .to_string()
    });
    rsx! {
        div {
            "unspendable_pk: {unspendable_pk}"
            "\n"
            "\n"
            "txid: {txid.cloned().unwrap_or_default()}"
            Button { text: "Click Me!" }
        }
    }
}
