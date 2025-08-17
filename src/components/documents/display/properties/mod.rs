use dioxus::logger::tracing::{debug, info};
use dioxus::prelude::*;
use sea_orm::{ActiveModelTrait, Set};

use crate::components::documents::display::{
    DocumentSignal,
    LocationsSignal,
    OrganizationsSignal,
    PersonsSignal,
};
use crate::database::get_database;
use crate::entities::DocumentActiveModel;
use crate::entities::document::{Keywords, Languages, Persons, ReviewStatus};
use crate::utils::language::Language;

mod basic;
mod keywords;
mod locations;
mod organisations;
mod persons;

use basic::{InputFilename, InputLanguages, InputName, InputReview};
use keywords::InputKeywords;
use locations::InputLocations;
use organisations::InputOrganisations;
use persons::InputPersons;

async fn submit(mut document: DocumentActiveModel, event: Event<FormData>) {
    debug!("Event: {event:?}");

    let values = event.values();

    document.title = Set(values["title"].as_value());

    // TODO: Handle organizations
    // TODO: Handle locations

    match values.get("persons") {
        Some(persons) => {
            document.persons = Set(Persons(
                persons
                    .as_slice()
                    .iter()
                    .map(|person| person.trim())
                    .filter(|person| !person.is_empty())
                    .map(String::from)
                    .collect(),
            ))
        }
        None => document.persons = Set(Persons(vec![])),
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

    let database = get_database().await;
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
                class: "space-y-4 pb-1 min-w-fit",
                li { InputFilename { document } }
                li { InputName { document } }
                li { InputPersons { document, persons } }
                li { InputOrganisations { organizations } }
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
