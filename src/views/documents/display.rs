use anyhow::Result;
use dioxus::html::{button, option};
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use sea_orm::{EntityTrait, ModelTrait};
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::{Set};
use strum::IntoEnumIterator;
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
use crate::utils::date::Calendar;
use crate::utils::language::Language;
use crate::utils::read_input::parse_input;

use crate::views::documents::display_panes::input_fields;

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

            let document2 = use_signal(|| document.clone());

            // rsx! {
        // Some(Ok(Some((document, location, locations, organizations, persons)))) =>
        // {
            // let mut keywords = use_signal(move || {document.keywords.0.clone()});
            rsx! {
            document::Link { rel: "stylesheet", href: asset!("/assets/styles/urejanje.css") },
            document::Script { src: asset!("/assets/scripts/grid.js") },
            div { class: "trije_divi panes pane h-full",
                div { class: "leva_stran pane",
                input_fields::element {
                    document: document.clone(),
                    location: location.clone(),
                },
                // form {
                // //     onsubmit: async move |event| { parse_input(event) },
                //     ul{
                //         li {
                //             label { "Ime datoteke:" }
                //             label { "{document.filename}" }
                //         }
                //         li {
                //             label { "Naslov dokumenta: "}
                //             input { name : "title", value: "{document.title}"}
                //         }
                //         li {
                //             label {"Datum: "} //TODO: Kakšen format ima datum
                //             input { name: "date", value: "{document.date.map_or(\"\".to_string(), |date| date.to_string())}" }
                //             select {
                //                 name: "calendar",
                //                 option {
                //                     value: "Gregor",
                //                     "Gregorijanski"
                //                 }
                //                 option {
                //                     value: "Julijan",
                //                     "Julijanski"
                //                 }
                //             }
                //         }
                //         li {
                //             label {"Imena oseb: "}

                //             button {
                //                 // onclick: |event| println!("clicked {event:?}" ), "Gumb"
                //             }
                //         }
                //         // li {
                //         //     // label {"imena oseb"}
                //         //     // ul {
                //         //     //     padding_left: "30px",
                //         //     //     list_styler_type: "square",

                //         //     //     //glavno ime
                //         //     //     li {
                //         //     //         input { value: document.find_related(Person).all(get_database().await).await.unwrap() }
                //         //     //     }


                //         //         //TODO: Format bo drugačen ko bo implementiran v bazi
                //         //         // for name in [Vec::from(["ime1osebe1", "ime2osebe2"]), Vec::from(["oseba2"]), Vec::from(["filip", "še en filip", "pravzaprav so tu kar trije filipi"]), Vec::from(["zdaj se je pa pojavila še ena vida"])] {
                //         //         //     li {
                //         //         //         list_styler_type: "square",
                //         //         //         ul {
                //         //         //             for variacije in name {
                //         //         //                 //TODO: variacije v svojem text area
                //         //         //                 li {
                //         //         //                     input {
                //         //         //                         n"ime",
                //         //         //                         spellcheck: "false",
                //         //         //                         value: "{variacije}"
                //         //         //                     }
                //         //         //                 }
                //         //         //             }
                //         //         //         }
                //         //         //     }
                //         //         // }
                //         //     }
                //         // }
                //         li {
                //             label { "Lokacija: " }
                //             //TODO: Glavna lokacija i ostale lokacije
                //             input { name: "main_location", value: "{location.clone().map_or(\"\".to_string(), |location| location.name)}" }
                //         }
                //         li {
                //             // for klb in keywords {
                //             //     input {
                //             //             name: "neki",
                //             //             value: "{klb}",
                //             //             onkeypress: |event| {
                //             //                 if event.key() == Enter {
                //             //                     event.prevent_default();
                //             //                     println!("Enter pressed");
                //             //                 }
                //             //             }
                //             //         }
                //             //     }
                //             label {"Ključne besede: "}

                //             input {
                //                 name: "neq_keyword",
                //                 value: "",
                //                 onkeypress: move |event: Event<KeyboardData>| {
                //                     if event.key() == Enter {
                //                         event.prevent_default();
                //                         keywords.push(String::new());
                //                         println!("Enter pressed {:?}", &event);
                //                         println!("Keywords: {:?}", keywords);
                //                     }
                //                 }
                //             }


                //             // for klb in ["1", "2", "3"] {
                //             //     input {
                //             //         name: "keyword",
                //             //         value: "En value",
                //             //         onkeypress: |event| {
                //             //             if event.key() == Enter {
                //             //                 event.prevent_default();
                //             //                 println!("Enter pressed");
                //             //             }
                //             //         }
                //             //     }
                //             // }
                //             // textarea { name: "keyword", value: vec_to_multyline(document.keywords.0.clone())}
                //         }
                //         li {
                //             label {"Jeziki"}
                //             ul {
                //                 padding_left: "10px",
                //                 for jezik in Language::iter() {
                //                 li {input {
                //                     r#type: "checkbox",
                //                     value: "{jezik.two_letter_code()}",
                //                     // name: "{jezik.name()}",
                //                     name: "language"
                //                 }
                //                 label { "{jezik.name()}" }}
                //             }}

                //         }
                //         li {
                //             button {
                //                 "Shrani"
                //             }
                //         }
                //     }

                // }
                }

                div {
                    class: "srednja_stran pane",
                    form {
                        onsubmit: move |event: Event<FormData>| async move {
                            let mut document: crate::entities::document::ActiveModel = document2().clone().into();

                            let values = event.values();
                            document.summary = Set(values["summary"].as_value());
                            document.metadata = Set(values["metadata"].as_value());
                            document.content = Set(values["content"].as_value());

                            let database = get_database().await;
                            document.update(database).await.unwrap(); // TODO: Handle error

                            info!("Submitted! {event:?}")
                        },
                        textarea {
                            height: "200px",
                            width: "100%",
                            resize: "vertical",
                            autocapitalize: "false",
                            autocomplete: "false",
                            spellcheck: "false",
                            name: "summary",
                            value: "{document.summary}"
                        }
                        textarea {
                            height: "200px",
                            width: "100%",
                            resize: "vertical",
                            autocapitalize: "false",
                            autocomplete: "false",
                            spellcheck: "false",
                            name: "metadata",
                            value: "{document.metadata}"
                        }
                        textarea {
                            height: "450px",
                            width: "100%",
                            autocapitalize: "false",
                            autocomplete: "false",
                            spellcheck: "false",
                            name: "content",
                            value: "{document.content}"
                        }
                        input {
                            height: "20px",
                            r#type: "submit",
                            value: "Shrani"
                        }
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
