use dioxus::prelude::*;

#[component]
pub fn InputSwitch(
    id: &'static str,
    label_off: &'static str,
    label_on: &'static str,
    #[props(default = false)] checked: bool,
    #[props(default = false)] readonly: bool,
) -> Element {
    rsx! {
        label {
            class: "label",
            "{label_off}"
            input {
                class: "toggle toggle-xs",
                type: "checkbox",
                name: id,
                id: id,
                checked,
                readonly,
            }
            "{label_on}"
        }
    }
}
