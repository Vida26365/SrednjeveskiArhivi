use anyhow::Result;
use dioxus::prelude::*;
use sea_orm::{EntityTrait, ModelTrait};
use uuid::Uuid;

use crate::components::alerts::AlertError;
use crate::components::documents::{PaneInput, PanePdf, PaneText};
use crate::database::get_database;
use crate::entities::document::DocumentToPrimaryLocation;
use crate::entities::{
    Document,
    Location,
    LocationAlias,
    Organization,
    OrganizationAlias,
    Person,
    PersonAlias,
};

#[component]
pub fn DocumentDisplay(id: Uuid) -> Element {
    let document: Resource<Result<_>> = use_resource(move || async move {
        let database = get_database().await;

        match Document::find_by_id(id).one(database).await? {
            Some(document) => {
                let location =
                    document.find_linked(DocumentToPrimaryLocation).one(database).await?;

                let locations = document
                    .find_related(Location)
                    .find_with_related(LocationAlias)
                    .all(database)
                    .await?;

                let organizations = document
                    .find_related(Organization)
                    .find_with_related(OrganizationAlias)
                    .all(database)
                    .await?;

                let persons = document
                    .find_related(Person)
                    .find_with_related(PersonAlias)
                    .all(database)
                    .await?;

                Ok(Some((document, location, locations, organizations, persons)))
            }

            None => Ok(None),
        }
    });

    match &*document.read_unchecked() {
        Some(Ok(Some((document, location, locations, organizations, persons)))) => {
            let document = use_signal(|| document.clone());
            let location = use_signal(|| location.clone());
            let locations = use_signal(|| locations.clone());
            let organizations = use_signal(|| organizations.clone());
            let persons = use_signal(|| persons.clone());

            rsx! {
                link { rel: "stylesheet", href: asset!("/assets/styles/grid.css") },
                script { src: asset!("/assets/scripts/grid.js") },

                div {
                    class: "panes",
                    div {
                        class: "pane px-3 pt-2 pb-4",
                        PaneInput { document, location, locations, organizations, persons }
                    }
                    div {
                        class: "pane px-3 pt-2 pb-4",
                        PaneText { document }
                    }
                    div {
                        class: "pane",
                        PanePdf { document }
                    }
                }
            }
        }
        Some(Ok(None)) => rsx! {
            AlertError {
                title: "Dokument ni najden".to_string(),
                details: "".to_string(),
            }
        },
        Some(Err(error)) => rsx! {
            AlertError {
                title: "Napaka pri nalaganju dokumenta".to_string(),
                details: format!("{error:?}"),
            }
        },
        None => rsx! {
            "Nalaganje dokumenta ..."
        },
    }
}
