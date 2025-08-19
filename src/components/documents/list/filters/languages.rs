use dioxus::prelude::*;
use sea_orm::Iterable;

use crate::utils::language::Language;
use crate::utils::text::capitalize;

#[component]
pub fn FilterLanguages() -> Element {
    rsx! {
        label {
            class: "flex pb-2 font-semibold",
            "Jeziki"
        }
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
            label {
                class: "label",
                "Unija"
                input {
                    class: "toggle toggle-xs",
                    type: "checkbox",
                    name: "languages-condition",
                    id: "languages-condition",
                }
                "Presek"
            }
        }
    }
}
