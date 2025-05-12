use dioxus::prelude::*;

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
                        path { d: "M3 16.5v2.25A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75V16.5m-13.5-9L12 3m0 0 4.5 4.5M12 3v13.5" }
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
                        path { d: "M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.325.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l1.296 2.247a1.125 1.125 0 0 1-.26 1.431l-1.003.827c-.293.241-.438.613-.43.992a7.723 7.723 0 0 1 0 .255c-.008.378.137.75.43.991l1.004.827c.424.35.534.955.26 1.43l-1.298 2.247a1.125 1.125 0 0 1-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.47 6.47 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.281c-.09.543-.56.94-1.11.94h-2.594c-.55 0-1.019-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.369-.49l-1.297-2.247a1.125 1.125 0 0 1 .26-1.431l1.004-.827c.292-.24.437-.613.43-.991a6.932 6.932 0 0 1 0-.255c.007-.38-.138-.751-.43-.992l-1.004-.827a1.125 1.125 0 0 1-.26-1.43l1.297-2.247a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.086.22-.128.332-.183.582-.495.644-.869l.214-1.28Z" }
                        path { d: "M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" }
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
