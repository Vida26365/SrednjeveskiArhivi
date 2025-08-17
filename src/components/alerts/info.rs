use dioxus::prelude::*;

#[component]
pub fn AlertInfo(
    #[props(into)] title: String,
    #[props(into, default = "")] details: String,
) -> Element {
    super::render_alert(
        &title,
        &details,
        "alert-info",
        "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
    )
}
