use dioxus::logger::tracing::{debug, info};
use dioxus::prelude::*;
use sea_orm::{ActiveModelTrait, Set};
use uuid::Uuid;

use crate::components::documents::display::{
    DocumentSignal,
    LocationsSignal,
    OrganizationsSignal,
    PersonsSignal,
};
use crate::database::get_database;
use crate::entities::{DocumentActiveModel, DocumentPersonActiveModel, PersonActiveModel, PersonAliasActiveModel};
use crate::entities::document::{Keywords, Languages, Persons, ReviewStatus};
use crate::utils::language::Language;

mod basic;
mod keywords;
mod list_inputov_generator;

use basic::{InputFilename, InputPersons, InputOrganizations, InputLocations, InputLanguages, InputName, InputReview};
use keywords::InputKeywords;
// pub use list_inputov_generator::{SublistInputList, LastInputOziromaVaskiPosebnez};

async fn submit(mut document: DocumentActiveModel, event: Event<FormData>) {
    let database = get_database().await;

    debug!("Event: {event:?}");

    let values = event.values();

    document.title = Set(values["title"].as_value());

    // TODO: Handle organizations
    // TODO: Handle locations

      match values.get("persons") {
        Some(osebe) => {
            for oseba in osebe.as_slice() {
                let person = PersonActiveModel {
                    id: Set(Uuid::now_v7()),
                    name: Set(oseba.trim().to_string()),
                    description: Set(String::new()),
                };
                let person = person.insert(database).await.unwrap();
                let document_person = DocumentPersonActiveModel {
                    document: Set(document.clone().id.unwrap()),
                    person: Set(person.id),
                };
                document_person.insert(database).await.unwrap();
                match values.get(&format!("oseba/{oseba}")) {
                    Some(variacije) => {
                        for variacija in variacije.as_slice() {
                            let alias = PersonAliasActiveModel {
                                id: Set(Uuid::now_v7()),
                                person: Set(Some(person.id)),
                                name: Set(variacija.trim().to_string()),
                                description: Set(String::new()),
                            };
                            alias.insert(database).await.unwrap();
                        }
                    }
                    None => {}
                }
            }
        }
        None => {}
    }

    match values.get("keywords") {
        Some(keywords) => {
            document.keywords = Set(Keywords(
                keywords
                    .as_slice()
                    .iter()
                    .map(|kw| kw.trim())
                    .filter(|kw| !kw.is_empty())
                    .map(String::from)
                    .collect(),
            ))
        }
        None => document.keywords = Set(Keywords(vec![])),
    }

    match values.get("languages") {
        Some(languages) => {
            document.languages = Set(Languages(
                languages
                    .as_slice()
                    .iter()
                    .filter_map(|lang| Language::from_two_letter_code(lang))
                    .collect(),
            ))
        }
        None => document.languages = Set(Languages(vec![])),
    }

    match ReviewStatus::from_variant_name(&values["review"].as_value()) {
        Some(review) => document.review = Set(review),
        None => document.review = Set(ReviewStatus::NotReviewed),
    }

    debug!("Parsed: {document:?}");

    document.update(database).await.unwrap(); // TODO: Handle errors

    info!("Submitted!"); // TODO: Show success message
}

#[component]
pub fn PaneProperties(
    #[props(into)] document: DocumentSignal,
    #[props(into)] locations: LocationsSignal,
    #[props(into)] organizations: OrganizationsSignal,
    #[props(into)] persons: PersonsSignal,
) -> Element {
    rsx! {
        form {
            onsubmit: move |event| async move {
                submit(document.read().clone().into(), event).await;
            },
            ul {
                class: "space-y-4 pb-1",
                li { InputFilename { document } }
                li { InputName { document } }
                li { InputPersons { document, persons } }
                li { InputOrganizations { organizations } }
                li { InputLocations { locations } }
                li { InputKeywords { document } }
                li { InputLanguages { document } }
                li { InputReview { document } }
                li {
                    button {
                        class: "btn btn-soft btn-primary rounded-box",
                        "Shrani"
                    }
                }
            }
        }
    }
}
