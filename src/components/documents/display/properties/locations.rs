use dioxus::prelude::*;
use dioxus_heroicons::IconShape;
use dioxus_heroicons::outline::Shape;

use crate::components::documents::display::LocationsSignal;

#[component]
pub fn InputLocations(locations: LocationsSignal) -> Element {
    let locations = use_signal(move || {
        locations
            .read()
            .clone()
            .into_iter()
            .map(|(_, lokacije)| {
                lokacije.iter().map(|lokacija| lokacija.name.clone()).collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    });
    rsx! {
        label {
            class: "flex pb-2 font-semibold",
            "Lokacije"
        }
        for location in locations.read().iter() {
            // TODO
        }
        div {
            class: "flex gap-2",
            div {
                class: "input w-full",
                input {
                    aria_autocomplete: "none",
                    autocapitalize: "false",
                    autocomplete: "false",
                    spellcheck: "false",
                    name: "locations",
                }
            }
        }
    }
}
