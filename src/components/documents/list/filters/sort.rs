use dioxus::prelude::*;

#[component]
pub fn Sorter() -> Element {
    rsx! {
        label {
            class: "flex pb-2 font-semibold",
            "Razvrstitev"
        }
        fieldset {
            class: "flex gap-2",
            select {
                class: "select w-full",
                name: "sort-column",
                id: "sort-column",
                option { value: "title", "Naslov" }
                option { value: "date", "Datum" }
                option { value: "location", "Kraj" }
                option { value: "review", "Stanje" }
            }
            select {
                class: "select w-full",
                name: "sort-order",
                id: "sort-order",
                option { value: "ascending", "Naraščajoče" }
                option { value: "descending", "Padajoče" }
            }
        }
    }
}
