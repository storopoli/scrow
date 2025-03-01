//! Shared navbar component.

use dioxus::prelude::*;

use crate::Route;

/// Shared navbar component.
#[component]
pub(crate) fn Navbar() -> Element {
    rsx! {
        nav {
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div { class: "flex justify-between h-16",
                    div { class: "flex items-center sm:hidden",
                        button {
                            r#type: "button",
                            class: "inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500",
                            aria_controls: "mobile-menu",
                            aria_expanded: "false",
                            span { class: "sr-only", "Menu" }
                            i { class: "fas fa-bars block h-6 w-6" }
                        }
                    }
                    div { class: "flex",
                        div { class: "flex-shrink-0 flex items-center",
                            span { class: "text-xl font-bold text-gray-900", "Satoshi Escrow" }
                        }
                        div { class: "hidden sm:ml-6 sm:flex sm:space-x-8",
                            Link {
                                id: "home",
                                class: "border-indigo-500 text-gray-900 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium",
                                to: Route::Home {},
                                "Home"
                            }
                            Link {
                                id: "create",
                                class: "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium",
                                to: Route::Create {},
                                "Create"
                            }
                            Link {
                                id: "sign",
                                class: "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium",
                                to: Route::Home {}, // TODO: Sign
                                "Sign"
                            }
                            Link {
                                id: "combine",
                                class: "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium",
                                to: Route::Home {}, // TODO: Combine
                                "Combine"
                            }
                            Link {
                                id: "broadcast",
                                class: "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium",
                                to: Route::Home {}, // TODO: Broadcast
                                "Broadcast"
                            }
                            Link {
                                id: "spend",
                                class: "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium",
                                to: Route::Home {}, // TODO: Spend
                                "Spend"
                            }
                        }
                    }
                    div { class: "flex",
                        Link {
                            id: "settings",
                            class: "p-2 rounded-full text-gray-500 hover:text-gray-700 focus:outline-none",
                            to: Route::Home {}, // TODO: Settings
                            i { class: "fas fa-cog text-xl" }
                            span { class: "sr-only", "Settings" }
                        }
                    }
                }
            }
            div { id: "mobile-menu", class: "sm:hidden",
                div { class: "pt-2 pb-3 space-y-1",
                    Link {
                        id: "home",
                        class: "bg-indigo-50 border-indigo-500 text-indigo-700 block pl-3 pr-4 py-2 border-l-4 text-base font-medium",
                        aria_current: "page",
                        to: Route::Home {},
                        "Home"
                    }
                    Link {
                        id: "create",
                        class: "border-transparent text-gray-600 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-800 block pl-3 pr-4 py-2 border-l-4 text-base font-medium",
                        to: Route::Home {}, // TODO: Create
                        "Create"
                    }
                    Link {
                        id: "sign",
                        class: "border-transparent text-gray-600 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-800 block pl-3 pr-4 py-2 border-l-4 text-base font-medium",
                        to: Route::Home {}, // TODO: Sign
                        "Sign"
                    }
                    Link {
                        id: "combine",
                        class: "border-transparent text-gray-600 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-800 block pl-3 pr-4 py-2 border-l-4 text-base font-medium",
                        to: Route::Home {}, // TODO: Combine
                        "Combine"
                    }
                    Link {
                        id: "broadcast",
                        class: "border-transparent text-gray-600 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-800 block pl-3 pr-4 py-2 border-l-4 text-base font-medium",
                        to: Route::Home {}, // TODO: Broadcast
                        "Broadcast"
                    }
                    Link {
                        id: "spend",
                        class: "border-transparent text-gray-600 hover:bg-gray-50 hover:border-gray-300 hover:text-gray-800 block pl-3 pr-4 py-2 border-l-4 text-base font-medium",
                        to: Route::Home {}, // TODO: Spend
                        "Spend"
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}
