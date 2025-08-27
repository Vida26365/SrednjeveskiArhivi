use dioxus::prelude::*;

use crate::components::documents::display::PrimaryLocationSignal;

#[component]
pub fn InputLocation(location: PrimaryLocationSignal) -> Element {
    rsx! {
        input {
            class: "input w-full",
            placeholder: "Kraj",
            aria_autocomplete: "none",
            autocapitalize: "false",
            autocomplete: "false",
            spellcheck: "false",
            name: "primary-location",
            id: "primary-location",
            value: "{location.read().as_ref().map_or(\"\", |location| &location.name)}",
        }
    }
}
