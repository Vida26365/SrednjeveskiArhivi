use dioxus::prelude::*;

#[component]
pub fn AlertError(
    #[props(into)] title: String,
    #[props(into, default = "")] details: String,
) -> Element {
    rsx! {
        div {
            class: "alert alert-soft alert-error",
            role: "alert",
            svg {
                class: "h-6 w-6 stroke-current",
                fill: "none",
                path {
                    d: "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                }
            }
            strong { "{title}" }
        }
        if !details.is_empty() {
            div {
                class: "alert alert-soft alert-error mt-2",
                role: "alert",
                div { class: "h-6 w-6" }
                p { class: "whitespace-pre-line", "{details}" }
            }
        }
    }
}
