use dioxus::prelude::*;

use crate::Route;

/// Shared navbar component.
#[component]
pub(crate) fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            class: "navbar bg-base-100 shadow-sm",
            div {
                class: "navbar-start",
                div {
                    class: "dropdown",
                    div {
                        tabindex:"0", role:"button", class: "btn btn-ghost lg:hidden",
                        svg {
                            xmlns:"http://www.w3.org/2000/svg", class:"h-5 w-5", fill:"none", view_box:"0 0 24 24", stroke:"currentColor",
                            path {
                                stroke_linecap:"round", stroke_linejoin:"round", stroke_width:"2", d:"M4 6h16M4 12h8m-8 6h16"
                            }
                        }
                    }
                    ul {
                        tabindex: "0",
                        class: "menu menu-sm dropdown-content bg-base-100 rounded-box z-1 mt-3 w-52 p-2 shadow",
                        li {
                            Link {
                                to: Route::Home {},
                                "Home"
                            }
                        }
                    }
                }
                a {
                    class: "btn btn-ghost text-xl",
                    Link {
                        to: Route::Home {},
                        "Home"
                    }
                }
            }
            div {
                class: "navbar-end",
                a {
                   class: "btn" ,
                   href: "https://github.com/storopoli/scrow",
                   "GitHub"
                }
            }
        }
        Outlet::<Route> {}
    }
}
