use dioxus::prelude::*;
use sea_orm::EntityTrait;

use crate::components::alerts::error::AlertError;
use crate::components::alerts::info::AlertInfo;
use crate::components::alerts::success::AlertSuccess;
use crate::components::alerts::warning::AlertWarning;
use crate::database::get_database;
use crate::entities::Document;

#[component]
pub fn Documents() -> Element {
    let documents = use_resource(|| async move {
        let database = get_database().await;
        Document::find().all(database).await
    });

    rsx! {
        div { class: "p-4",
            {
                match &*documents.read() {
                    Some(Ok(docs)) => rsx! {
                        div {
                            // DaisyUI responsive table styling
                            div { class: "overflow-x-auto",
                                table { class: "table w-full",
                                    thead {
                                        tr {
                                            th { "Filename" }
                                            th { "Title" }
                                            th { "Keywords" }
                                            th { "Summary" }
                                            th { "Content" }
                                        }
                                    }
                                    tbody {
                                        {docs.iter().map(|doc| rsx! {
                                            tr {
                                                td { "{doc.filename}" }
                                                td { "{doc.title}" }
                                                // td { "{doc.keywords}.0.join(\", \")" }
                                                td { "{doc.summary.as_deref().unwrap_or(\"\")}" }
                                                td { "{doc.content.as_deref().unwrap_or(\"\")}" }
                                            }
                                        })}
                                    }
                                }
                            }
                        }
                    },
                    Some(Err(err)) => rsx! {
                        AlertInfo { message: "Error loading documents".to_string(), details: err.to_string() }
                    },
                    None => rsx! {
                        div { class: "text-center mt-4", "Loading documents..." }
                    },
                }
            }
        }
    }
}
