use dioxus::prelude::*;

#[component]
pub fn AlertWarning(
    #[props(into)] title: String,
    #[props(into, default = "")] details: String,
) -> Element {
    rsx! {
        div {
            class: "alert alert-soft alert-warning",
            role: "alert",
            svg {
                class: "h-6 w-6 stroke-current",
                fill: "none",
                path {
                    d: "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                }
            }
            strong { "{title}" }
        }
        if !details.is_empty() {
            div {
                class: "alert alert-soft alert-warning mt-2",
                role: "alert",
                div { class: "h-6 w-6" }
                p { class: "whitespace-pre-line", "{details}" }
            }
        }
    }
}
