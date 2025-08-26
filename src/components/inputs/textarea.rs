use dioxus::prelude::*;

#[component]
pub fn InputTextarea(
    id: &'static str,
    #[props(into)] value: Option<String>,
    #[props(into)] placeholder: Option<String>,
    #[props(default = false)] readonly: bool,
) -> Element {
    rsx! {
        textarea {
            class: "textarea autoresize w-full",
            aria_autocomplete: "none",
            autocapitalize: "false",
            autocomplete: "false",
            spellcheck: "false",
            name: id,
            id: id,
            value,
            placeholder,
            readonly,
        }
    }
}
