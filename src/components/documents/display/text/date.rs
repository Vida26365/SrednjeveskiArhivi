use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::components::documents::display::DocumentSignal;
use crate::utils::date::Calendar;
use crate::utils::text::capitalize;

#[component]
pub fn InputDate(document: DocumentSignal) -> Element {
    rsx! {
        input {
            class: "input w-full",
            placeholder: "Datum",
            aria_autocomplete: "none",
            autocapitalize: "false",
            autocomplete: "false",
            spellcheck: "false",
            name: "date",
            id: "date",
            value: "{document.read().date.map_or(\"\".to_string(), |date| date.to_string())}",
        }
        select {
            class: "select w-full",
            name: "calendar",
            id: "calendar",
            for calendar in Calendar::iter() {
                option {
                    value: "{calendar.as_variant_name()}",
                    selected: "{document.read().date.map_or(false, |date| date.calendar() == calendar)}",
                    "{capitalize(&calendar.to_string())}"
                }
            }
        }
    }
}
