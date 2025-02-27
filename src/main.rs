//! Satoshi Escrow Dixous App

use bitcoin::{Address, Network};
use components::Button;
use dioxus::prelude::*;
use esplora::{create_client, get_funding_txid};
use scripts::UNSPENDABLE_PUBLIC_KEY;

pub mod components;
pub mod error;
pub mod esplora;
pub mod scripts;
pub mod sign;
pub mod tx;
pub mod util;

#[cfg(debug_assertions)]
use dioxus::logger::{
    self,
    tracing::{Level, info},
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
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
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/daisyui@4.12.24/dist/full.min.css",
            integrity: "sha384-2V5uSMIWpBK7suX6yRDZH6ll7ktPJF2O58y0HSz+HiFCBCsmqZpxX1AZB4qAHuYI",
            crossorigin: "anonymous"
        }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
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

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}

    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Blog { id: 1 },
                "Blog"
            }
        }

        Outlet::<Route> {}
    }
}
