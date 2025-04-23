use dioxus::prelude::*;

#[component]
pub fn AlertSuccess(title: String, details: String) -> Element {
    rsx! {
        div {
            class: "alert alert-soft alert-success",
            role: "alert",
            svg {
                class: "h-6 w-6 stroke-current",
                fill: "none",
                path {
                    d: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                }
            }
            div {
                p { strong { "{title}" }}
                p { "{details}" }
            }
        }
    }
}
