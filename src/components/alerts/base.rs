use dioxus::prelude::*;

pub fn render_alert(title: &str, details: &str, class: &str, path: &str) -> Element {
    rsx! {
        div {
            div {
                class: "alert alert-soft {class}",
                role: "alert",
                svg {
                    class: "h-6 w-6 stroke-current",
                    fill: "none",
                    path {
                        d: path,
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                    }
                }
                strong { "{title}" }
            }
            if !details.is_empty() {
                div {
                    class: "alert alert-soft {class} mt-2",
                    role: "alert",
                    div { class: "h-6 w-6" }
                    p { class: "whitespace-pre-line", "{details}" }
                }
            }
        }
    }
}
