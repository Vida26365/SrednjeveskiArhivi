use dioxus::prelude::*;
use dioxus::logger::tracing::info;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::{Set};

use crate::entities::document::Model as DocumentModel;
use crate::database::get_database;




#[component]
pub fn element(document: DocumentModel) -> Element {
    let document2 = use_signal(|| document.clone());

    rsx! {
        link { rel: "stylesheet", href: asset!("/assets/styles/urejanje.css") },
        link { rel: "stylesheet", href: asset!("/assets/styles/function.css") },
        script { src: asset!("/assets/scripts/auto_grow.js") },

        form {
            onsubmit: move |event: Event<FormData>| async move {
                            let mut document: crate::entities::document::ActiveModel = document2().clone().into();

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
                                value: "{document.summary}"
                            }
                        }
                        div {
                            class: "grow-wrap",
                            textarea {
                                width: "100%",
                                resize: "vertical",
                                autocapitalize: "false",
                                autocomplete: "false",
                                spellcheck: "false",
                                name: "metadata",
                                value: "{document.metadata}"
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
                                value: "{document.content}"
                            }
                        }
                        button { "Shrani" }

        }
    }
}

// #[component]


