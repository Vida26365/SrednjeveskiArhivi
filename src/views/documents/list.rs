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
                                th { "Osebe" }
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
                                        class: "space-x-1 space-y-1",
                                        for person in &document.persons.0 {
                                            span {
                                                class: "badge badge-soft",
                                                "{person}"
                                            }
                                        }
                                    }
                                    td {
                                        class: "space-x-1 space-y-1",
                                        for keyword in &document.keywords.0 {
                                            span {
                                                class: "badge badge-soft",
                                                "{keyword}"
                                            }
                                        }
                                    }
                                    td {
                                        class: "text-nowrap",
                                        match document.review {
                                            ReviewStatus::NotReviewed => rsx! {
                                                span {
                                                    class: "badge badge-soft badge-warning",
                                                    "{ReviewStatus::NotReviewed}"
                                                }
                                            },
                                            ReviewStatus::UnderReview => rsx! {
                                                span {
                                                    class: "badge badge-soft badge-info",
                                                    "{ReviewStatus::UnderReview}"
                                                }
                                            },
                                            ReviewStatus::Reviewed => rsx! {
                                                span {
                                                    class: "badge badge-soft badge-success",
                                                    "{ReviewStatus::Reviewed}"
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
