//! Shared navbar component.

use dioxus::prelude::*;

use crate::{LOGO, Route};

/// Shared navbar component.
#[component]
pub(crate) fn Navbar() -> Element {
    // Get the current route
    let route = use_route::<Route>();

    // Helper function to determine if a link is active
    let is_active = |path: Route| -> bool { route == path };

    // Add state to track if mobile menu is open
    let mut is_menu_open = use_signal(|| false);

    rsx! {
        nav {
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div { class: "flex justify-between h-16",
                    div { class: "flex items-center sm:hidden",
                        button {
                            r#type: "button",
                            class: "inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500",
                            aria_controls: "mobile-menu",
                            aria_expanded: "{is_menu_open}",
                            onclick: move |_| is_menu_open.set(!is_menu_open()),
                            span { class: "sr-only", "Menu" }
                            i {
                                // Show different icon based on menu state
                                if *is_menu_open.read() {
                                    // X icon when menu is open
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
                                        class: "lucide lucide-x",

                                        line {
                                            x1: "18",
                                            x2: "6",
                                            y1: "6",
                                            y2: "18",
                                        }
                                        line {
                                            x1: "6",
                                            x2: "18",
                                            y1: "6",
                                            y2: "18",
                                        }
                                    }
                                } else {
                                    // Hamburger icon when menu is closed
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
                                        class: "lucide lucide-menu",

                                        line {
                                            x1: "4",
                                            x2: "20",
                                            y1: "12",
                                            y2: "12",
                                        }
                                        line {
                                            x1: "4",
                                            x2: "20",
                                            y1: "6",
                                            y2: "6",
                                        }
                                        line {
                                            x1: "4",
                                            x2: "20",
                                            y1: "18",
                                            y2: "18",
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "flex",
                        div { class: "flex-shrink-0 flex items-center",
                            img {
                                src: LOGO,
                                alt: "Satoshi Escrow Logo",
                                class: "h-12 w-12 mr-2",
                            }
                            span { class: "text-xl font-bold text-gray-900", "Satoshi Escrow" }
                        }
                        div { class: "hidden sm:ml-6 sm:flex sm:space-x-8",
                            Link {
                                id: "home",
                                class: if is_active(Route::Home {}) { "border-indigo-500 text-gray-900 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium" } else { "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium" },
                                aria_current: if is_active(Route::Home {}) { "page" } else { "" },
                                to: Route::Home {},
                                "Home"
                            }
                            Link {
                                class: if is_active(Route::Create {}) { "border-indigo-500 text-gray-900 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium" } else { "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium" },
                                aria_current: if is_active(Route::Create {}) { "page" } else { "" },
                                to: Route::Create {},
                                "Create"
                            }
                            Link {
                                id: "sign",
                                class: if is_active(Route::Sign {}) { "border-indigo-500 text-gray-900 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium" } else { "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium" },
                                aria_current: if is_active(Route::Sign {}) { "page" } else { "" },
                                to: Route::Sign {},
                                "Sign"
                            }
                            Link {
                                id: "combine",
                                class: if is_active(Route::Combine {}) { "border-indigo-500 text-gray-900 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium" } else { "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium" },
                                aria_current: if is_active(Route::Combine {}) { "page" } else { "" },
                                to: Route::Combine {},
                                "Combine"
                            }
                            Link {
                                id: "broadcast",
                                class: if is_active(Route::Broadcast {}) { "border-indigo-500 text-gray-900 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium" } else { "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium" },
                                aria_current: if is_active(Route::Broadcast {}) { "page" } else { "" },
                                to: Route::Broadcast {},
                                "Broadcast"
                            }
                            Link {
                                id: "spend",
                                class: if is_active(Route::Spend {}) { "border-indigo-500 text-gray-900 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium" } else { "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium" },
                                aria_current: if is_active(Route::Spend {}) { "page" } else { "" },
                                to: Route::Spend {},
                                "Spend"
                            }
                        }
                    }
                    div { class: "flex",
                        Link {
                            id: "settings",
                            class: if is_active(Route::Settings {}) { "p-2 rounded-full text-indigo-500 focus:outline-none" } else { "p-2 rounded-full text-gray-500 hover:text-gray-700 focus:outline-none" },
                            aria_current: if is_active(Route::Settings {}) { "page" } else { "" },
                            to: Route::Settings {},
                            i {
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
                                    class: "lucide lucide-cog",

                                    path { d: "M12 20a8 8 0 1 0 0-16 8 8 0 0 0 0 16Z" }
                                    path { d: "M12 14a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z" }
                                    path { d: "M12 2v2" }
                                    path { d: "M12 22v-2" }
                                    path { d: "m17 20.66-1-1.73" }
                                    path { d: "M11 10.27 7 3.34" }
                                    path { d: "m20.66 17-1.73-1" }
                                    path { d: "m3.34 7 1.73 1" }
                                    path { d: "M14 12h8" }
                                    path { d: "M2 12h2" }
                                    path { d: "m20.66 7-1.73 1" }
                                    path { d: "m3.34 17 1.73-1" }
                                    path { d: "m17 3.34-1 1.73" }
                                    path { d: "m11 13.73-4 6.93" }
                                }
                            }
                            span { class: "sr-only", "Settings" }
                        }
                    }
                }
            }
            div {
                id: "mobile-menu",
                class: if *is_menu_open.read() { "sm:hidden" } else { "hidden sm:hidden" },
                div { class: "pt-2 pb-3 space-y-1",
                    Link {
                        id: "home",
                        class: if is_active(Route::Home {}) { "bg-indigo-50 border-indigo-500 text-indigo-700 block pl-3 pr-4 py-2 border-l-4 text-base font-medium" } else { "border-transparent text-gray-600 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-800 block pl-3 pr-4 py-2 border-l-4 text-base font-medium" },
                        aria_current: if is_active(Route::Home {}) { "page" } else { "" },
                        onclick: move |_| {
                            *is_menu_open.write() = false;
                        },
                        to: Route::Home {},
                        "Home"
                    }
                    Link {
                        id: "create",
                        class: if is_active(Route::Create {}) { "bg-indigo-50 border-indigo-500 text-indigo-700 block pl-3 pr-4 py-2 border-l-4 text-base font-medium" } else { "border-transparent text-gray-600 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-800 block pl-3 pr-4 py-2 border-l-4 text-base font-medium" },
                        aria_current: if is_active(Route::Create {}) { "page" } else { "" },
                        onclick: move |_| {
                            *is_menu_open.write() = false;
                        },
                        to: Route::Create {},
                        "Create"
                    }
                    Link {
                        id: "sign",
                        class: if is_active(Route::Sign {}) { "bg-indigo-50 border-indigo-500 text-indigo-700 block pl-3 pr-4 py-2 border-l-4 text-base font-medium" } else { "border-transparent text-gray-600 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-800 block pl-3 pr-4 py-2 border-l-4 text-base font-medium" },
                        aria_current: if is_active(Route::Sign {}) { "page" } else { "" },
                        onclick: move |_| {
                            *is_menu_open.write() = false;
                        },
                        to: Route::Sign {},
                        "Sign"
                    }
                    Link {
                        id: "combine",
                        class: if is_active(Route::Combine {}) { "bg-indigo-50 border-indigo-500 text-indigo-700 block pl-3 pr-4 py-2 border-l-4 text-base font-medium" } else { "border-transparent text-gray-600 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-800 block pl-3 pr-4 py-2 border-l-4 text-base font-medium" },
                        aria_current: if is_active(Route::Combine {}) { "page" } else { "" },
                        onclick: move |_| {
                            *is_menu_open.write() = false;
                        },
                        to: Route::Combine {},
                        "Combine"
                    }
                    Link {
                        id: "broadcast",
                        class: if is_active(Route::Broadcast {}) { "bg-indigo-50 border-indigo-500 text-indigo-700 block pl-3 pr-4 py-2 border-l-4 text-base font-medium" } else { "border-transparent text-gray-600 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-800 block pl-3 pr-4 py-2 border-l-4 text-base font-medium" },
                        aria_current: if is_active(Route::Broadcast {}) { "page" } else { "" },
                        onclick: move |_| {
                            *is_menu_open.write() = false;
                        },
                        to: Route::Broadcast {},
                        "Broadcast"
                    }
                    Link {
                        id: "spend",
                        class: if is_active(Route::Spend {}) { "bg-indigo-50 border-indigo-500 text-indigo-700 block pl-3 pr-4 py-2 border-l-4 text-base font-medium" } else { "border-transparent text-gray-600 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-800 block pl-3 pr-4 py-2 border-l-4 text-base font-medium" },
                        aria_current: if is_active(Route::Spend {}) { "page" } else { "" },
                        onclick: move |_| {
                            *is_menu_open.write() = false;
                        },
                        to: Route::Spend {},
                        "Spend"
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}
