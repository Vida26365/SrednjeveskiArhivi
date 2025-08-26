use dioxus::prelude::*;
use sea_orm::Iterable;

use crate::components::inputs::{InputLabel, InputSwitch};
use crate::utils::language::Language;
use crate::utils::text::capitalize;

#[component]
pub fn FilterLanguages() -> Element {
    rsx! {
        InputLabel { title: "Jeziki" }
        fieldset {
            class: "space-y-2",
            for language in Language::iter() {
                div {
                    class: "whitespace-nowrap",
                    input {
                        class: "checkbox",
                        type: "checkbox",
                        name: "languages",
                        id: "languages-{language.as_two_letter_code()}",
                        value: "{language.as_two_letter_code()}",
                    }
                    label {
                        class: "ps-2",
                        for: "languages-{language.as_two_letter_code()}",
                        "{capitalize(language.as_name())}"
                    }
                }
            }
        }
        InputSwitch { id: "languages-condition", label_off: "Unija", label_on: "Presek" }
    }
}
