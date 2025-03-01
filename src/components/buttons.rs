//! Buttons Components.

#[cfg(debug_assertions)]
use dioxus::logger::tracing;
use dioxus::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

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
                    Err(e) => {
                        #[cfg(debug_assertions)]
                        tracing::error!(?e, "Failed to copy to clipboard");
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
            class: "inline-flex items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
            i { class: "fas fa-copy mr-2" }
            " Copy {text}"
        }
    }
}

/// Primary button component.
#[component]
pub fn PrimaryButton(text: String, onclick: Callback<MouseEvent>) -> Element {
    rsx! {
        button {
            r#type: "button",
            onclick,
            class: "inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
            i { class: "fas fa-check mr-2" }
            " {text}"
        }
    }
}
