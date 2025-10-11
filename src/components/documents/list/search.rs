use anyhow::Result;
use dioxus::logger::tracing::debug;
use dioxus::prelude::*;
use dioxus_heroicons::IconShape;
use dioxus_heroicons::outline::Shape;

use crate::components::alerts::AlertError;
use crate::components::documents::list::{DocumentSearch, SearchSignal};
use crate::components::inputs::InputText;

async fn submit(mut search: SearchSignal, event: Event<FormData>) -> Result<()> {
    debug!("Event: {event:?}");

    let search: &mut DocumentSearch = &mut search.write();
    let values = event.values();

    search.query = values["search-query"].as_value().trim().to_string();

    // match values["search-type"].as_value().as_str() {
    //     "combined" => search.r#type = DocumentSearchType::Combined,
    //     "text" => search.r#type = DocumentSearchType::Text,
    //     "vector" => search.r#type = DocumentSearchType::Vector,
    //     unknown => bail!("Invalid search type: {unknown}"),
    // }

    debug!("Parsed: {search:?}");

    Ok(())
}

#[component]
pub fn SearchBar(#[props(into)] search: SearchSignal) -> Element {
    let mut state: Signal<Option<Element>> = use_signal(|| None);

    rsx! {
        div {
            class: "space-y-2",
            form {
                class: "flex gap-2",
                onsubmit: move |event| async move {
                    match submit(search, event).await {
                        Ok(_) => state.set(None),
                        Err(error) => state.set(Some(rsx! {
                            AlertError {
                                title: "Napaka pri iskanju",
                                details: format!("{error:?}"),
                            }
                        })),
                    }
                },
                InputText {
                    id: "search-query",
                    placeholder: "Išči ...",
                }
                // select {
                //     class: "select",
                //     name: "search-type",
                //     id: "search-type",
                //     option { value: "combined", "Kombinirano" }
                //     option { value: "text", "Besedilno" }
                //     option { value: "vector", "Vektorsko" }
                // }
                button {
                    class: "btn btn-square btn-soft",
                    type: "submit",
                    svg {
                        class: "size-4",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        { Shape::MagnifyingGlass.path() }
                    }
                }
            }
            match state() {
                Some(element) => rsx! { { element } },
                None => rsx! {},
            }
        }
    }
}
