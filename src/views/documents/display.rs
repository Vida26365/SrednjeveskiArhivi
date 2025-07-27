use anyhow::Result;
use dioxus::prelude::*;
use sea_orm::{EntityTrait, ModelTrait};
use uuid::Uuid;

use crate::components::alerts::AlertError;
use crate::components::documents::display::{PaneInput, PanePdf, PaneText};
use crate::components::skeleton::Skeleton;
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

    rsx! {
        link { rel: "stylesheet", href: asset!("/assets/styles/grid.css") }
        script { src: asset!("/assets/scripts/grid.js") }

        div {
            class: "panes",
            div {
                class: "pane ps-3 pe-4 py-3",
                "data-default-size": 0.2,
                match &*document.read_unchecked() {
                    Some(Ok(Some((document, _, locations, organizations, persons)))) => rsx! {
                        PaneInput {
                            document: document.clone(),
                            locations: locations.clone(),
                            organizations: organizations.clone(),
                            persons: persons.clone(),
                        }
                    },
                    _ => rsx! {
                        Skeleton {}
                    }
                }
            }
            div {
                class: "pane px-4 py-3",
                match &*document.read_unchecked() {
                    Some(Ok(Some((document, location, _, _, _)))) => rsx! {
                        PaneText {
                            document: document.clone(),
                            location: location.clone(),
                        }
                    },
                    Some(Ok(None)) => rsx! {
                        AlertError {
                            title: "Dokument ni najden",
                        }
                    },
                    Some(Err(error)) => rsx! {
                        AlertError {
                            title: "Napaka pri nalaganju dokumenta",
                            details: format!("{error:?}"),
                        }
                    },
                    None => rsx! {
                        Skeleton {}
                    },
                }
            }
            div {
                class: "pane ps-1",
                "data-default-size": 0.3855,
                match &*document.read_unchecked() {
                    Some(Ok(Some((document, _, _, _, _)))) => rsx! {
                        PanePdf {
                            document: document.clone(),
                        }
                    },
                    _ => rsx! {
                        div {
                            class: "size-full ps-3 pe-3 py-3",
                            Skeleton {}
                        }
                    },
                }
            }
        }
    }
}
