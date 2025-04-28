use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            class: "navbar sticky top-0 z-50 bg-base-200",
            div {
                class: "navbar-start",
                Link {
                    class: "btn btn-ghost text-lg",
                    to: Route::DocumentList {},
                    "Dokumenti"
                }
                Link {
                    class: "btn btn-ghost text-lg",
                    to: Route::Blog { id: 1 },
                    "Blog"
                }
            }
        }

        main {
            class: "p-2",
            Outlet::<Route> {}
        }
    }
}
