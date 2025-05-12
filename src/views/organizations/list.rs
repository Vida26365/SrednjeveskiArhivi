use anyhow::Result;
use dioxus::prelude::*;
use sea_orm::EntityTrait;

use crate::components::alerts::error::AlertError;
use crate::database::get_database;
use crate::entities::{Document, Organization};
use crate::entities::document;

#[component]
fn FirstMention(documents: Vec<document::Model>) -> Element {
    let first_mention = documents
        .iter()
        .min_by_key(|document| document.date);

    rsx! { "TODO" }
}

#[component]
pub fn OrganizationList() -> Element {
    let organizations: Resource<Result<_>> = use_resource(|| async move {
        let database = get_database().await;
        Ok(Organization::find().find_with_related(Document).all(database).await?)
    });

    match &*organizations.read_unchecked() {
        Some(Ok(organizations)) => rsx! {
            div {
                div {
                    class: "overflow-x-auto overflow-y-auto",
                    table {
                        class: "table w-full",
                        thead {
                            tr {
                                th { "Ime" }
                                th { "Število omemb" }
                                th { "Prva omemba" }
                                th { "Zadnja omemba" }
                                th { "Dejanja" }
                            }
                        }
                        tbody {
                            for organization in organizations {
                                tr {
                                    "{organization:#?}"
                                }
                            }
                        }
                    }
                }
            }
        },
        Some(Err(error)) => rsx! {
            AlertError {
                title: "Napaka pri nalaganju organizacij".to_string(),
                details: format!("{:?}", error),
            }
        },
        None => rsx! {
            "Nalaganje organizacij ..."
        },
    }
}
