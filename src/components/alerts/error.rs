use dioxus::prelude::*;

#[component]
pub fn AlertError(
    #[props(into)] title: String,
    #[props(into, default = "")] details: String,
) -> Element {
    super::render_alert(
        &title,
        &details,
        "alert-error",
        "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z",
    )
}
