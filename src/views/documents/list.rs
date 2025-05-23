use anyhow::Result;
use dioxus::prelude::*;
use sea_orm::EntityTrait;

use crate::components::alerts::AlertError;
use crate::database::get_database;
use crate::entities::document::{DocumentToPrimaryLocation, ReviewStatus};
use crate::entities::Document;
use crate::Route;

#[component]
pub fn DocumentList() -> Element {
    let documents: Resource<Result<_>> = use_resource(async || {
        let database = get_database().await;
        Ok(Document::find().find_also_linked(DocumentToPrimaryLocation).all(database).await?)
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
                                th { "Datum" }
                                th { "Kraj" }
                                th { "Ključne besede" }
                                th { "Stanje" }
                                th { "Dejanja" }
                            }
                        }
                        tbody {
                            for (document, location) in documents {
                                tr {
                                    td { "{document.title}" }
                                    td { "{document.date.map_or(\"/\".to_string(), |date| date.to_string())}" }
                                    td { "{location.clone().map_or(\"/\".to_string(), |location| location.name)}" }
                                    td {
                                        for keyword in &document.keywords.0 {
                                            span {
                                                class: "badge badge-soft me-1",
                                                "{keyword}"
                                            }
                                        }
                                    }
                                    td {
                                        match document.review {
                                            ReviewStatus::NotReviewed => rsx! {
                                                span {
                                                    class: "badge badge-soft badge-warning",
                                                    "Ni pregledan"
                                                }
                                            },
                                            ReviewStatus::UnderReview => rsx! {
                                                span {
                                                    class: "badge badge-soft badge-info",
                                                    "V pregledu"
                                                }
                                            },
                                            ReviewStatus::Reviewed => rsx! {
                                                span {
                                                    class: "badge badge-soft badge-success",
                                                    "Pregledan"
                                                }
                                            },
                                        }
                                    }
                                    td {
                                        Link {
                                            to: Route::DocumentDisplay { id: document.id },
                                            class: "btn btn-soft btn-primary rounded-box",
                                            "Poglej"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
        Some(Err(error)) => rsx! {
            AlertError {
                title: "Napaka pri nalaganju dokumentov".to_string(),
                details: format!("{error:?}"),
            }
        },
        None => rsx! {
            "Nalaganje dokumentov ..."
        },
    }
}
