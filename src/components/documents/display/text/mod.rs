use dioxus::logger::tracing::{debug, info};
use dioxus::prelude::*;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;

use crate::components::documents::display::{DocumentSignal, PrimaryLocationSignal};
use crate::database::get_database;
use crate::entities::DocumentActiveModel;
use crate::utils::date::{Calendar, Date};

mod date;
mod location;

use date::InputDate;
use location::InputLocation;

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

    // TODO: Handle location

    debug!("Parsed: {document:?}");

    let database = get_database().await;
    document.update(database).await.unwrap(); // TODO: Handle errors

    info!("Submitted!"); // TODO: Show success message
}

#[component]
pub fn PaneText(
    #[props(into)] document: DocumentSignal,
    #[props(into)] location: PrimaryLocationSignal,
) -> Element {
    rsx! {
        link { rel: "stylesheet", href: asset!("/assets/styles/autoresize.css") }
        script { src: asset!("/assets/scripts/autoresize.js") }

        form {
            onsubmit: move |event| async move {
                submit(document.read().clone().into(), event).await;
            },
            ul {
                class: "space-y-4 pb-1 min-w-fit",
                li {
                    class: "flex gap-4",
                    InputLocation { location }
                    InputDate { document }
                }
                li {
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
                li {
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
                li {
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
                li {
                    button {
                        class: "btn btn-soft btn-primary rounded-box",
                        "Shrani"
                    }
                }
            }
        }
    }
}
