use dioxus::prelude::*;
use sea_orm::Iterable;

use crate::components::documents::display::{
    DocumentSignal,
    LocationsSignal,
    OrganizationsSignal,
    PersonsSignal,
};
use crate::components::inputs::{InputLabel, InputList, InputNestedList, InputText};
use crate::entities::document::ReviewStatus;
use crate::utils::language::Language;
use crate::utils::text::capitalize;

#[component]
pub fn InputId(document: DocumentSignal) -> Element {
    rsx! {
        InputLabel { title: "ID", target: "id" }
        InputText { id: "id", value: "{document.read().id}", readonly: true }
    }
}

#[component]
pub fn InputName(document: DocumentSignal) -> Element {
    rsx! {
        InputLabel { title: "Naslov", target: "title" }
        InputText { id: "title", value: "{document.read().title}" }
    }
}

#[component]
pub fn InputPersons(document: DocumentSignal, persons: PersonsSignal) -> Element {
    let persons: Vec<_> = persons
        .read()
        .iter()
        .map(|(person, aliases)| {
            (Some(person.name.clone()), aliases.iter().map(|alias| alias.name.clone()).collect())
        })
        .collect();

    rsx! {
        InputLabel { title: "Osebe" }
        InputNestedList { id: "persons", values: persons }
    }
}

#[component]
pub fn InputOrganizations(organizations: OrganizationsSignal) -> Element {
    let organizations: Vec<_> = organizations
        .read()
        .iter()
        .map(|(organization, aliases)| {
            (
                Some(organization.name.clone()),
                aliases.iter().map(|alias| alias.name.clone()).collect(),
            )
        })
        .collect();

    rsx! {
        InputLabel { title: "Organizacije" }
        InputNestedList { id: "organizations", values: organizations }
    }
}

#[component]
pub fn InputLocations(locations: LocationsSignal) -> Element {
    let locations: Vec<_> = locations
        .read()
        .iter()
        .map(|(location, aliases)| {
            (Some(location.name.clone()), aliases.iter().map(|alias| alias.name.clone()).collect())
        })
        .collect();

    rsx! {
        InputLabel { title: "Lokacije" }
        InputNestedList { id: "locations", values: locations }
    }
}

#[component]
pub fn InputKeywords(document: DocumentSignal) -> Element {
    let keywords = document.read().keywords.0.clone();

    rsx! {
        InputLabel { title: "Ključne besede" }
        InputList { id: "keywords", values: keywords }
    }
}

#[component]
pub fn InputLanguages(document: DocumentSignal) -> Element {
    rsx! {
        InputLabel { title: "Jeziki" }
        fieldset {
            class: "space-y-2",
            for language in Language::iter() {
                div {
                    class: "whitespace-nowrap",
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
pub fn InputReview(document: DocumentSignal) -> Element {
    rsx! {
        InputLabel { title: "Stanje" }
        fieldset {
            class: "space-y-2",
            for review in ReviewStatus::iter() {
                div {
                    class: "whitespace-nowrap",
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
