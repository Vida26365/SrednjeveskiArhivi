use dioxus::logger::tracing::{debug, info};
use dioxus::prelude::*;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;

use crate::database::get_database;
use crate::entities::{DocumentActiveModel, DocumentModel};

async fn submit(mut document: DocumentActiveModel, event: Event<FormData>) {
    debug!("Event: {event:?}");

    let values = event.values();
    document.summary = Set(values["summary"].as_value());
    document.metadata = Set(values["metadata"].as_value());
    document.content = Set(values["content"].as_value());

    debug!("Parsed: {document:?}");

    let database = get_database().await;
    document.update(database).await.unwrap(); // TODO: Handle errors

    info!("Submitted!"); // TODO: Show success message
}

#[component]
pub fn PaneText(document: Signal<DocumentModel>) -> Element {
    rsx! {
        link { rel: "stylesheet", href: asset!("/assets/styles/autoresize.css") }
        script { src: asset!("/assets/scripts/autoresize.js") }

        form {
            onsubmit: move |event| async move {
                submit(document.read().clone().into(), event).await;
            },
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
