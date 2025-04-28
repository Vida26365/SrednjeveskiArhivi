use dioxus::prelude::*;
use sea_orm::EntityTrait;

use crate::components::alerts::error::AlertError;
use crate::database::get_database;
use crate::entities::Document;
use crate::Route;

#[component]
pub fn DocumentList() -> Element {
    let documents = use_resource(|| async move {
        let database = get_database().await;
        Document::find().all(database).await
    });

    match &*documents.read_unchecked() {
        Some(Ok(documents)) => rsx! {
            div {
                div {
                    class: "overflow-x-auto overflow-y-auto",
                    table {
                        class: "table w-full",
                        thead {
                            tr {
                                th { "Naslov" }
                                th { "Datum"}
                                th { "Kraj" }
                                th { "Ključne besede" }
                                th { "Dejanja" }
                            }
                        }
                        tbody {
                            {documents.iter().map(|document| rsx! {
                                tr {
                                    td { "{document.title}" }
                                    td { "TODO" }
                                    td { "TODO" }
                                    td { "{document.keywords.0.join(\", \")}" }
                                    td {
                                        Link {
                                            to: Route::DocumentDisplay { id: document.id },
                                            class: "btn btn-primary",
                                            "Poglej"
                                        }
                                    }
                                }
                            })}
                        }
                    }
                }
            }
        },
        Some(Err(error)) => rsx! {
            AlertError {
                title: "Napaka pri nalaganju dokumentov".to_string(),
                details: error.to_string(),
            }
        },
        None => rsx! {
            "Nalaganje dokumentov ..."
        },
    }
}
