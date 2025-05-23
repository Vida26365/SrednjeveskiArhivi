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
        link { rel: "stylesheet", href: asset!("/assets/styles/function.css") },
        script { src: asset!("/assets/scripts/auto_grow.js") },

        form {
            onsubmit: move |event: Event<FormData>| async move {
                submit(document.read().clone().into(), event).await;
            },
            div {
                class: "grow-wrap",
                textarea {
                    height: "auto",
                    width: "100%",
                    autocapitalize: "false",
                    autocomplete: "false",
                    spellcheck: "false",
                    name: "summary",
                    value: "{document.read().summary}",
                }
            }
            div {
                class: "grow-wrap",
                textarea {
                    width: "100%",
                    autocapitalize: "false",
                    autocomplete: "false",
                    spellcheck: "false",
                    name: "metadata",
                    value: "{document.read().metadata}",
                }
            }
            div {
                class: "grow-wrap",
                textarea {
                    width: "100%",
                    autocapitalize: "false",
                    autocomplete: "false",
                    spellcheck: "false",
                    name: "content",
                    value: "{document.read().content}",
                }
            }
            button { "Shrani" }
        }
    }
}
