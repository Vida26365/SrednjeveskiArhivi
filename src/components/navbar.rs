use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            class: "navbar bg-base-200 sticky top-0",
            div {
                class: "navbar-start",
                Link {
                    class: "btn btn-ghost text-lg",
                    to: Route::Home {},
                    "Dokumenti"
                }
                Link {
                    class: "btn btn-ghost text-lg",
                    to: Route::Blog { id: 1 },
                    "Blog"
                }
            }
        }

        Outlet::<Route> {}
    }
}
