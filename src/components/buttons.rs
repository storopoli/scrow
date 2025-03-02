//! Buttons Components.

use dioxus::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

#[cfg(debug_assertions)]
use dioxus::logger::tracing;

use crate::Route;

/// Copy button component.
#[component]
pub(crate) fn CopyButton(text: String, clipboard_text: String) -> Element {
    // Get a handle to the clipboard
    let navigator = use_memo(move || window().map(|w| w.navigator()));

    #[cfg(debug_assertions)]
    tracing::trace!("Clipboard handle obtained");

    let on_copy = move |_| {
        let clipboard_text = clipboard_text.clone();
        spawn(async move {
            if let Some(navigator) = navigator() {
                let clipboard = navigator.clipboard();
                #[cfg(debug_assertions)]
                tracing::info!("Got clipboard access");

                match JsFuture::from(clipboard.write_text(&clipboard_text)).await {
                    Ok(_) => {
                        #[cfg(debug_assertions)]
                        tracing::info!(text = %clipboard_text, "Copied to clipboard");
                    }
                    Err(_e) => {
                        #[cfg(debug_assertions)]
                        tracing::error!(?_e, "Failed to copy to clipboard");
                    }
                }
            } else {
                #[cfg(debug_assertions)]
                tracing::error!("Clipboard not available");
            }
        });
    };
    rsx! {
        button {
            r#type: "button",
            onclick: on_copy,
            class: "ml-3 inline-flex items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
            i { class: "mr-1 sm:mr-2",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "24",
                    height: "24",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    "stroke-width": "2",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                    class: "lucide lucide-copy",

                    rect {
                        width: "14",
                        height: "14",
                        x: "8",
                        y: "8",
                        rx: "2",
                        ry: "2",
                    }
                    path { d: "M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" }
                }
            }
            " Copy {text}"
        }
    }
}

/// Primary button component.
#[component]
pub(crate) fn PrimaryButton(text: String, onclick: Callback<MouseEvent>) -> Element {
    rsx! {
        button {
            r#type: "button",
            onclick,
            class: "ml-3 inline-flex items-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
            "{text}"
        }
    }
}

/// Secondary button component.
#[component]
pub(crate) fn SecondaryButton(text: String, onclick: Callback<MouseEvent>) -> Element {
    rsx! {
        button {
            r#type: "button",
            onclick,
            class: "ml-3 inline-flex justify-center py-2 px-4 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
            "{text}"
        }
    }
}

/// Continue to section button component.
#[component]
pub(crate) fn ContinueButton(text: String, to: Route) -> Element {
    rsx! {
        Link {
            class: "ml-3 inline-flex items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
            to,
            i { class: "mr-1 sm:mr-2",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    width: "24",
                    height: "24",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    "stroke-width": "2",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                    class: "lucide lucide-check",

                    path { d: "M20 6 9 17l-5-5" }
                }
            }
            " {text}"
        }
    }
}
