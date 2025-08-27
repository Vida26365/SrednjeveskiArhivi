use dioxus::prelude::*;

#[component]
pub fn InputLabel(
    title: &'static str,
    target: Option<&'static str>,
    #[props(default = false)] readonly: bool,
) -> Element {
    rsx! {
        label {
            class: "flex font-semibold",
            for: target,
            "{title}"
        }
    }
}
