use anyhow::Result;
use dioxus::prelude::*;
use sea_orm::EntityTrait;

use crate::components::alerts::AlertError;
use crate::components::documents::list::{PaneFilters, PaneTable};
use crate::components::skeleton::Skeleton;
use crate::database::get_database;
use crate::entities::document::DocumentToPrimaryLocation;
use crate::entities::Document;

#[component]
pub fn DocumentList() -> Element {
    let documents: Resource<Result<_>> = use_resource(async || {
        let database = get_database().await;
        Ok(Document::find().find_also_linked(DocumentToPrimaryLocation).all(database).await?)
    });

    rsx! {
        link { rel: "stylesheet", href: asset!("/assets/styles/grid.css") }
        script { src: asset!("/assets/scripts/grid.js") }

        div {
            class: "panes",
            div {
                class: "pane ps-3 pe-4 py-3",
                "data-default-size": 0.2,
                PaneFilters {}
            }
            div {
                class: "pane ps-4 pe-3 py-3",
                match &*documents.read_unchecked() {
                    Some(Ok(documents)) => rsx! {
                        PaneTable { documents: documents.clone() }
                    },
                    Some(Err(error)) => rsx! {
                        AlertError {
                            title: "Napaka pri nalaganju dokumentov",
                            details: format!("{error:?}"),
                        }
                    },
                    None => rsx! {
                        Skeleton {}
                    },
                }
            }
        }
    }
}
