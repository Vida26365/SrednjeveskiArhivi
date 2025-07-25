use dioxus::logger::tracing::{debug, info};
use dioxus::prelude::*;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;

use crate::database::get_database;
use crate::entities::{DocumentActiveModel, DocumentModel, LocationModel};
use crate::utils::date::{Calendar, Date};

type LocationParam = Signal<Option<LocationModel>>;
type DocumentParam = Signal<DocumentModel>;

async fn submit(mut document: DocumentActiveModel, event: Event<FormData>) {
    debug!("Event: {event:?}");

    let values = event.values();
    document.summary = Set(values["summary"].as_value());
    document.metadata = Set(values["metadata"].as_value());
    document.content = Set(values["content"].as_value());
    if values["date"].as_value().trim() == "" {
        document.date = Set(None);
    } else {
        // TODO: Handle errors
        let calendar = Calendar::from_variant_name(&values["calendar"].as_value()).unwrap();
        let date = Date::parse(&values["date"].as_value(), &calendar).unwrap();
        document.date = Set(Some(date));
    }
    // TOdo: Handle location

    debug!("Parsed: {document:?}");

    let database = get_database().await;
    document.update(database).await.unwrap(); // TODO: Handle errors

    info!("Submitted!"); // TODO: Show success message
}

#[component]
pub fn PaneText(document: Signal<DocumentModel>, location: LocationParam) -> Element {
    rsx! {
        link { rel: "stylesheet", href: asset!("/assets/styles/autoresize.css") }
        script { src: asset!("/assets/scripts/autoresize.js") }

        form {
            onsubmit: move |event| async move {
                submit(document.read().clone().into(), event).await;
            },
            div {
                class: "flex gap-8",
                InputDate { document }
                InputLocations { location }
            }
            div {
                class: "mb-4",
                label {
                    class: "flex pb-2 font-semibold",
                    for: "summary",
                    "Povzetek"
                }
                textarea {
                    class: "textarea autoresize w-full",
                    aria_autocomplete: "none",
                    autocapitalize: "false",
                    autocomplete: "false",
                    spellcheck: "false",
                    name: "summary",
                    id: "summary",
                    value: "{document.read().summary}",
                }
            }
            div {
                class: "mb-4",
                label {
                    class: "flex pb-2 font-semibold",
                    for: "metadata",
                    "Metapodatki"
                }
                textarea {
                    class: "textarea autoresize w-full",
                    aria_autocomplete: "none",
                    autocapitalize: "false",
                    autocomplete: "false",
                    spellcheck: "false",
                    name: "metadata",
                    id: "metadata",
                    value: "{document.read().metadata}",
                }
            }
            div {
                class: "mb-4",
                label {
                    class: "flex pb-2 font-semibold",
                    for: "content",
                    "Vsebina"
                }
                textarea {
                    class: "textarea autoresize w-full",
                    aria_autocomplete: "none",
                    autocapitalize: "false",
                    autocomplete: "false",
                    spellcheck: "false",
                    name: "content",
                    id: "content",
                    value: "{document.read().content}",
                }
            }
            button {
                class: "btn btn-soft btn-primary rounded-box",
                "Shrani"
            }
        }
    }
}

#[component]
fn InputLocations(location: LocationParam) -> Element {
    rsx! {
        input {
            class: "input w-full",
            placeholder: "Lokacija",
            aria_autocomplete: "none",
            autocapitalize: "false",
            autocomplete: "false",
            spellcheck: "false",
            name: "main-location",
            id: "main-location",
            value: "{location.read().clone().map_or(\"\".to_string(), |location| location.name)}",
        }
    }
}

#[component]
fn InputDate(document: DocumentParam) -> Element {
    rsx! {
        input {
            class: "input mb-2 w-full",
            placeholder: "Datum",
            aria_autocomplete: "none",
            autocapitalize: "false",
            autocomplete: "false",
            spellcheck: "false",
            name: "date",
            id: "date",
            value: "{document.read().date.map_or(\"\".to_string(), |date| date.to_string())}",
        }
        // fieldset {
        //     class: "space-y-2",
        //     for calendar in Calendar::iter() {
        //         div {
        //             input {
        //                 class: "radio",
        //                 type: "radio",
        //                 name: "calendar",
        //                 id: "calendar-{calendar.as_variant_name()}",
        //                 value: "{calendar.as_variant_name()}",
        //                 checked: "{document.read().date.map_or(false, |date| date.calendar() == calendar)}",
        //             }
        //             label {
        //                 class: "ps-2",
        //                 for: "calendar-{calendar.as_variant_name()}",
        //                 "{capitalize(&calendar.to_string())}"
        //             }
        //         }
        //     }
        // }
    }
}
