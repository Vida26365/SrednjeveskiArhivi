use dioxus::prelude::*;

#[component]
pub fn AlertInfo(title: String, details: String) -> Element {
    rsx! {
        div {
            class: "alert alert-soft alert-info",
            role: "alert",
            svg {
                class: "h-6 w-6 stroke-current",
                fill: "none",
                path {
                    d: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
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
