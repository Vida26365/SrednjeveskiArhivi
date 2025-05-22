use anyhow::Result;
use dioxus::prelude::*;
use sea_orm::{EntityTrait, ModelTrait};
use sea_orm::ActiveValue::{Set};
use uuid::Uuid;

use crate::components::alerts::error::AlertError;
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

use crate::views::documents::display_panes::{input_display, text_display};
// use crate::views::documents::display_panes::text_display;

// https://stackoverflow.com/questions/53777136/dynamic-html-form-elements-as-array

fn vec_to_multyline(vec: Vec<String>) -> String {
    let mut value = String::new();
    for key in &vec {
        value += &(String::from("\n") + key)
    }
    value
}

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
            // let document = document.clone();

            rsx! {
            link { rel: "stylesheet", href: asset!("/assets/styles/urejanje.css") },
            script { src: asset!("/assets/scripts/grid.js") },
            div { class: "trije_divi panes pane h-full",
                div { class: "leva_stran pane",
                    input_display::element {
                        document: document.clone(),
                        location: location.clone(),
                    }
                }

                div {
                    class: "srednja_stran pane",
                    text_display::element {
                        document: document.clone(),
                    }
                }

                div {
                    class: "desna_stran pane",
                    embed {
                        src: "/content/{document.id}#toolbar=0",
                        type: "application/pdf",
                        width: "100%",
                        height: "100%",
                    }

            }

            }
        }
    },
        Some(Ok(None)) => rsx! {
            AlertError {
                title: "Dokument ni najden".to_string(),
                details: "".to_string(),
            }
        },
        Some(Err(error)) => rsx! {
            AlertError {
                title: "Napaka pri nalaganju dokumenta".to_string(),
                details: format!("{:?}", error),
            }
        },
        None => rsx! {
            "Nalaganje dokumenta ..."
        },
    }
}
