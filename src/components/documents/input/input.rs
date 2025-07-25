use dioxus::logger::tracing::{debug, info};
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use dioxus_heroicons::IconShape;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, Iterable};
use strum::IntoEnumIterator;

use crate::components::documents::input::{InputKeywords, InputOrganisations, InputPersons};
use crate::database::get_database;
use crate::entities::document::{Keywords, Languages, Persons, ReviewStatus};
use crate::entities::{
    DocumentActiveModel,
    DocumentModel,
    LocationAliasModel,
    LocationModel,
    OrganizationAliasModel,
    OrganizationModel,
    PersonAliasModel,
    PersonModel,
};
use crate::utils::language::Language;

type DocumentParam = Signal<DocumentModel>;
type LocationsParam = Signal<Vec<(LocationModel, Vec<LocationAliasModel>)>>;
type OrganizationsParam = Signal<Vec<(OrganizationModel, Vec<OrganizationAliasModel>)>>;
type PersonsParam = Signal<Vec<(PersonModel, Vec<PersonAliasModel>)>>;

fn capitalize(str: &str) -> String {
    let mut chars = str.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

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
            ));
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
pub fn PaneInput(
    document: DocumentParam,
    locations: LocationsParam,
    organizations: OrganizationsParam,
    persons: PersonsParam,
) -> Element {
    rsx! {
        form {
            onsubmit: move |event| async move {
                submit(document.read().clone().into(),event).await;
            },
            ul {
                class: "space-y-4",
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

#[component]
fn InputLocations(locations: LocationsParam) -> Element {
    let mut locations = use_signal(move || {
        locations
            .read()
            .clone()
            .into_iter()
            .map(|(_, lokacije)| {
                lokacije.iter().map(|lokacija| lokacija.name.clone()).collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    });
    rsx! {
        label {
            class: "flex pb-2 font-semibold",
            "Lokacije"
        }
        for location in locations.read().iter() {
            // TODO
        }
        div {
            class: "flex gap-2",
            div {
                class: "input w-full",
                input {
                    aria_autocomplete: "none",
                    autocapitalize: "false",
                    autocomplete: "false",
                    spellcheck: "false",
                    name: "locations",
                }
            }
        }
    }
}

#[component]
fn InputFilename(document: DocumentParam) -> Element {
    rsx! {
        label {
            class: "flex pb-2 font-semibold",
            for: "filename",
            "Ime datoteke"
        }
        input {
            class: "input w-full",
            aria_autocomplete: "none",
            autocapitalize: "false",
            autocomplete: "false",
            spellcheck: "false",
            readonly: "true",
            name: "filename",
            id: "filename",
            value: "{document.read().filename}",
        }
    }
}

#[component]
fn InputName(document: DocumentParam) -> Element {
    rsx! {
        label {
            class: "flex pb-2 font-semibold",
            for: "title",
            "Naslov"
        }
        input {
            class: "input w-full",
            aria_autocomplete: "none",
            autocapitalize: "false",
            autocomplete: "false",
            spellcheck: "false",
            name: "title",
            id: "title",
            value: "{document.read().title}",
        }
    }
}

#[component]
fn InputLanguages(document: DocumentParam) -> Element {
    rsx! {
        label {
            class: "flex pb-2 font-semibold",
            "Jeziki"
        }
        fieldset {
            class: "space-y-2",
            for language in Language::iter() {
                div {
                    input {
                        class: "checkbox",
                        type: "checkbox",
                        name: "languages",
                        id: "languages-{language.as_two_letter_code()}",
                        value: "{language.as_two_letter_code()}",
                        checked: "{document.read().languages.0.contains(&language)}",
                    }
                    label {
                        class: "ps-2",
                        for: "languages-{language.as_two_letter_code()}",
                        "{capitalize(language.as_name())}"
                    }
                }
            }
        }
    }
}

#[component]
fn InputReview(document: DocumentParam) -> Element {
    rsx! {
        label {
            class: "flex pb-2 font-semibold",
            "Stanje"
        }
        fieldset {
            class: "space-y-2",
            for review in ReviewStatus::iter() {
                div {
                    input {
                        class: "radio",
                        type: "radio",
                        name: "review",
                        id: "review-{review.as_variant_name()}",
                        value: "{review.as_variant_name()}",
                        checked: "{document.read().review == review}",
                    }
                    label {
                        class: "ps-2",
                        for: "review-{review.as_variant_name()}",
                        "{review}"
                    }
                }
            }
        }
    }
}
