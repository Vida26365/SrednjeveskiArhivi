use dioxus::prelude::*;
use sea_orm::Iterable;

use crate::utils::date::Calendar;
use crate::utils::text::capitalize;

#[component]
pub fn FilterDate() -> Element {
    rsx! {
        label {
            class: "flex pb-2 font-semibold",
            "Datum"
        }
        fieldset {
            class: "space-y-2",
            label {
                class: "input w-full validator-invalid",
                span {
                    class: "label min-w-20",
                    "Začetni"
                }
                input {
                    aria_autocomplete: "none",
                    autocapitalize: "false",
                    autocomplete: "false",
                    spellcheck: "false",
                    name: "start-date",
                    id: "start-date",
                    pattern: "\\d+\\. ?\\d+\\. ?\\d+",
                    title: "Datum v obliki dd. mm. yyyy",
                }
            }
            label {
                class: "input w-full validator-invalid",
                span {
                    class: "label min-w-20",
                    "Končni"
                }
                input {
                    aria_autocomplete: "none",
                    autocapitalize: "false",
                    autocomplete: "false",
                    spellcheck: "false",
                    name: "end-date",
                    id: "end-date",
                    pattern: "\\d+\\. ?\\d+\\. ?\\d+",
                    title: "Datum v obliki dd. mm. yyyy",
                }
            }
            select {
                class: "select w-full",
                name: "calendar",
                id: "calendar",
                for calendar in Calendar::iter() {
                    option {
                        value: "{calendar.as_variant_name()}",
                        "{capitalize(&calendar.to_string())}"
                    }
                }
            }
        }
    }
}
