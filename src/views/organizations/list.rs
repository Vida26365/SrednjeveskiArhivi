use anyhow::Result;
use dioxus::prelude::*;
use sea_orm::EntityTrait;

use crate::components::alerts::AlertError;
use crate::components::mentions::{MentionFirst, MentionLast};
use crate::database::get_database;
use crate::entities::{Document, Organization};

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
                            for (organization, documents) in organizations {
                                tr {
                                    td { "{organization.name}" }
                                    td { "{documents.len()}" }
                                    td { MentionFirst { documents: documents.clone() } }
                                    td { MentionLast { documents: documents.clone() } }
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
