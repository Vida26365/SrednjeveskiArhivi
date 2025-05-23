use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;

use crate::database::get_database;
use crate::entities::{DocumentActiveModel, DocumentModel};

#[component]
pub fn PaneText(document: Signal<DocumentModel>) -> Element {
    rsx! {
        link { rel: "stylesheet", href: asset!("/assets/styles/function.css") },
        script { src: asset!("/assets/scripts/auto_grow.js") },

        form {
            onsubmit: move |event: Event<FormData>| async move {
                let mut document: DocumentActiveModel = document.read().clone().into();

                let values = event.values();
                document.summary = Set(values["summary"].as_value());
                document.metadata = Set(values["metadata"].as_value());
                document.content = Set(values["content"].as_value());

                let database = get_database().await;
                document.update(database).await.unwrap(); // TODO: Handle error

                info!("Submitted! {event:?}")
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
