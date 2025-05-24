use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use dioxus_heroicons::IconShape;

use crate::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            class: "navbar sticky top-0 z-50 bg-base-200",
            div {
                class: "navbar-start space-x-2",
                Link {
                    class: "btn btn-primary btn-ghost aria-[current='page']:btn-soft rounded text-lg",
                    to: Route::DocumentList {},
                    "Dokumenti"
                }
                Link {
                    class: "btn btn-primary btn-ghost aria-[current='page']:btn-soft rounded text-lg",
                    to: Route::OrganizationList {},
                    "Organizacije"
                }
            }

            div {
                class: "navbar-end space-x-2",
                Link {
                    class: "btn btn-primary btn-ghost aria-[current='page']:btn-soft btn-circle",
                    to: Route::DocumentUpload {},
                    title: "Nalaganje",
                    alt: "Nalaganje",
                    svg {
                        class: "size-6",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "1.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        view_box: "0 0 24 24",
                        { Shape::ArrowUpTray.path() }
                    }
                }
                div {
                    class: "btn btn-primary btn-ghost aria-[current='page']:btn-soft btn-circle",
                    title: "Nastavitve",
                    alt: "Nastavitve",
                    svg {
                        class: "size-6",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "1.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        view_box: "0 0 24 24",
                        { Shape::Cog6Tooth.path() }
                    }
                }
            }
        }

        main {
            class: "p-2",
            Outlet::<Route> {}
        }
    }
}
