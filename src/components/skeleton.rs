use dioxus::prelude::*;

#[component]
pub fn Skeleton() -> Element {
    rsx! {
        div {
            class: "size-full animate-fade-in",
            div { class: "size-full skeleton" }
        }
    }
}
