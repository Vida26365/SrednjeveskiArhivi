use dioxus::events::Key::Enter;
use dioxus::prelude::*;
use strum::IntoEnumIterator;
use sea_orm::{EntityTrait, ModelTrait};
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::{Set};
use dioxus::logger::tracing::info;
use crate::database::get_database;

use crate::entities::document::Model as DocumentModel;
use crate::entities::location::Model as LocationModel;
use crate::entities::{location, Document, Organization, OrganizationAlias, Person, PersonAlias};
use crate::utils::language::Language;
use crate::utils::read_input::parse_input;

#[component]
pub fn element(document: DocumentModel, location: Option<LocationModel>) -> Element {
    let document2 = use_signal(|| document.clone());


    rsx! {
        form {
            onsubmit: move |event: Event<FormData>| async move {
                let mut document: crate::entities::document::ActiveModel = document2().into();

                let values = event.values();
                document.title = Set(values["title"].as_value());

                let database = get_database().await;
                document.update(database).await.unwrap(); // TODO: Handle error

                info!("Submitted! {event:?}")

            },
            ul {
                li { Filename {document: document.clone()} }
                li { DocumentName {document: document.clone()} }
                li { Date {document: document.clone()} }
                li { Persons {document: document.clone()} }
                li { Keywords {document: document.clone()} }
                li { Lokacija {location: location.clone()} }
                li { Languages {} }
                li {
                    button { "Shrani "}
                }
            }
        }
    }
}

#[component]
fn Filename(document: DocumentModel) -> Element {
    rsx! {
        label { "Ime datoteke:" }
        label { "{document.filename}" }
    }
}

#[component]
fn DocumentName(document: DocumentModel) -> Element {
    rsx! {
        label { "Naslov dokumenta: "}
        input { name : "title", value: "{document.title}"}
    }
}

#[component]
fn Date(document: DocumentModel) -> Element {
    rsx! {
        label {"Datum: "} //TODO: Kakšen format ima datum
        input { name: "date", value: "{document.date.map_or(\"\".to_string(), |date| date.to_string())}" }
        select {
            name: "calendar",
            option {
                value: "Gregor",
                "Gregorijanski"
            }
            option {
                value: "Julijan",
                "Julijanski"
            }
        }
    }
}

#[component]
fn Persons(document: DocumentModel) -> Element {
    rsx! {
        label { "Imena oseb:" }
        label { "TODO" }
    }
}

#[component]
fn Lokacija(location: Option<LocationModel>) -> Element {
    rsx! {
        label { "Lokacija: " }
        //TODO: Glavna lokacija in ostale lokacije
        input {
            name: "main_location",
            value: "{location.clone().map_or(\"\".to_string(), |location| location.name)}"
        }
    }
}

#[component]
fn Keywords(document: DocumentModel) -> Element {
    // TODO: Make thid work
    // let mut keywords = use_(move || {document.keywords.0.clone()});
    let mut keywords = document.keywords.0.clone();
    rsx! {
        label {"Ključne besede: "}

        for klb in keywords.clone() { //TODO: Lepša rešitev za vse
            input {
                name: "neki",
                value: "{klb}",
                onkeypress: |event| {
                    if event.key() == Enter {
                        event.prevent_default();
                        println!("Enter pressed");
                    }
                }
            }
        }

        input {
            name: "new_keyword",
            value: "",
            onkeypress: move |event: Event<KeyboardData>| {
                if event.key() == Enter {
                    // event.prevent_default();
                    keywords.push(String::new());
                    println!("Enter pressed {:?}", &event);
                    println!("Keywords: {:?}", keywords);
                }
            }
        }
    }
}

#[component]
fn Languages() -> Element {
    rsx! {
        label {"Jeziki"}
        ul {
            padding_left: "10px",
            for jezik in Language::iter() {
                li {
                    input {
                        r#type: "checkbox",
                        value: "{jezik.as_two_letter_code()}",
                        // name: "{jezik.name()}",
                        name: "language"
                    }
                    label { "{jezik.as_name()}" }
                }
            }
        }
    }
}
