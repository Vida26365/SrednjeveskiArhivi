use dioxus::prelude::*;

use crate::Route;
use crate::components::documents::list::DocumentsSignal;
use crate::entities::document::ReviewStatus;

#[component]
pub fn PaneTable(#[props(into)] documents: DocumentsSignal) -> Element {
    rsx! {
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
                        for (document, location) in documents.read().iter() {
                            tr {
                                td { "{document.title}" }
                                td { "{document.date.map_or(\"/\".to_string(), |date| date.to_string())}" }
                                td { "{location.clone().map_or(\"/\".to_string(), |location| location.name)}" }
                                td {
                                    span {
                                        class: "flex flex-wrap gap-1",
                                        for keyword in &document.keywords.0 {
                                            span {
                                                class: "badge badge-soft",
                                                "{keyword}"
                                            }
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
                                        class: "badge badge-soft badge-primary",
                                        "Poglej"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
