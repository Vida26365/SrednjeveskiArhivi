use dioxus::prelude::*;
use dioxus_heroicons::IconShape;
use dioxus_heroicons::outline::Shape;

use crate::Route;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum NavbarTab {
    // Button Tabs
    Documents,
    Persons,
    Organizations,
    Locations,
    // Icon Tabs
    Upload,
    Ocr,
    Settings,
}

impl NavbarTab {
    fn current(route: &Route) -> Self {
        match route {
            Route::DocumentList {} => Self::Documents,
            Route::DocumentDisplay { .. } => Self::Documents,
            // Route::PersonList {} => Self::Persons,
            // Route::PersonDisplay { .. } => Self::Persons,
            Route::OrganizationList {} => Self::Organizations,
            // Route::OrganizationDisplay { .. } => Self::Organizations,
            // Route::LocationList {} => Self::Locations,
            // Route::LocationDisplay { .. } => Self::Locations,
            Route::DocumentUpload {} => Self::Upload,
            Route::OcrBuiltin {} => Self::Ocr,
            Route::OcrExternalExport {} => Self::Ocr,
            Route::OcrExternalImport {} => Self::Ocr,
        }
    }
}

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            class: "navbar sticky top-0 z-50 bg-base-200",

            div {
                class: "navbar-start space-x-2",
                NavbarText {
                    to: Route::DocumentList {},
                    tab: NavbarTab::Documents,
                    title: "Dokumenti",
                }
                // NavbarText {
                //     to: Route::PersonList {},
                //     tab: NavbarTab::Persons,
                //     title: "Osebe",
                // }
                NavbarText {
                    to: Route::OrganizationList {},
                    tab: NavbarTab::Organizations,
                    title: "Organizacije",
                }
                // NavbarText {
                //     to: Route::LocationList {},
                //     tab: NavbarTab::Locations,
                //     title: "Lokacije",
                // }
            }

            div {
                class: "navbar-end space-x-2",
                NavbarIcon {
                    to: Route::DocumentUpload {},
                    tab: NavbarTab::Upload,
                    icon: Shape::ArrowUpTray,
                    title: "Dodajanje",
                }
                NavbarMenu {
                    tab: NavbarTab::Ocr,
                    icon: Shape::DocumentMagnifyingGlass,
                    title: "OCR",
                    items: vec![
                        (Route::OcrBuiltin {}, "Vgrajena prepoznava besedila"),
                        (Route::OcrExternalExport {}, "Izvozi za zunanjo prepoznavo"),
                        (Route::OcrExternalImport {}, "Uvozi iz zunanje prepoznave"),
                    ],
                }
                // NavbarIcon {
                //     to: Route::Settings {},
                //     tab: NavbarTab::Settings,
                //     icon: Shape::Cog6Tooth,
                //     title: "Nastavitve",
                // }
            }
        }

        main {
            class: "p-3 h-[calc(100vh-64px)] w-full overflow-auto",
            Outlet::<Route> {}
        }
    }
}

#[component]
fn NavbarText(to: Route, tab: NavbarTab, title: &'static str) -> Element {
    let active = NavbarTab::current(&use_route());
    let style = if active == tab { "btn-soft" } else { "btn-ghost" };

    rsx! {
        Link {
            class: "btn btn-primary {style} rounded-box text-lg",
            to: to,
            "{title}"
        }
    }
}

#[component]
fn NavbarIcon(to: Route, tab: NavbarTab, icon: Shape, title: &'static str) -> Element {
    let active = NavbarTab::current(&use_route());
    let style = if active == tab { "btn-soft" } else { "btn-ghost" };

    rsx! {
        Link {
            class: "btn btn-primary {style} btn-circle",
            to: to,
            title: "{title}",
            alt: "{title}",
            svg {
                class: "size-6",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                view_box: "0 0 24 24",
                { icon.path() }
            }
        }
    }
}

#[component]
fn NavbarMenu(
    tab: NavbarTab,
    icon: Shape,
    title: &'static str,
    items: Vec<(Route, &'static str)>,
) -> Element {
    let active = NavbarTab::current(&use_route());
    let style = if active == tab { "btn-soft" } else { "btn-ghost" };

    rsx! {
        div {
            class: "dropdown dropdown-end",
            div {
                class: "btn btn-primary {style} btn-circle",
                tabindex: "0",
                title: "{title}",
                alt: "{title}",
                svg {
                    class: "size-6",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "1.5",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    view_box: "0 0 24 24",
                    { icon.path() }
                }
            }
            ul {
                class: "dropdown-content menu mt-1 w-max bg-base-300 rounded-box shadow",
                tabindex: "0",
                for (to, title) in items {
                    li {
                        Link {
                            class: "aria-[current='page']:menu-active",
                            to: to,
                            "{title}"
                        }
                    }
                }
            }
        }
    }
}
