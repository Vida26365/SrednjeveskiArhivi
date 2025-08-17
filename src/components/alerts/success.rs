use dioxus::prelude::*;

#[component]
pub fn AlertSuccess(
    #[props(into)] title: String,
    #[props(into, default = "")] details: String,
) -> Element {
    super::render_alert(
        &title,
        &details,
        "alert-success",
        "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z",
    )
}
