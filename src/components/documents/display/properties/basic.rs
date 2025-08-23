use dioxus::prelude::*;
use sea_orm::Iterable;

use crate::components::documents::display::{DocumentSignal, LocationsSignal, OrganizationsSignal, PersonsSignal};
use crate::components::documents::display::properties::list_inputov_generator::{LastInputOziromaVaskiPosebnez, SublistInputList};
use crate::entities::document::ReviewStatus;
use crate::utils::language::Language;
use crate::utils::text::capitalize;

#[component]
pub fn InputFilename(document: DocumentSignal) -> Element {
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
pub fn InputName(document: DocumentSignal) -> Element {
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
pub fn InputPersons(document: DocumentSignal, persons: PersonsSignal) -> Element {
    let persons = use_signal(move || {
        persons
            .read()
            .clone()
            .into_iter()
            .map(|(person, aliases)| {
                (person.name, aliases.iter().map(|alias| alias.name.clone()).collect::<Vec<_>>())
            })
            .collect::<Vec<_>>()
    });

    rsx! {
        label {
            class: "flex pb-2 font-semibold",
            "Osebe"
        }
        SublistInputList {
            name: "persons".to_string(),
            string_vec_list: persons
        }
        LastInputOziromaVaskiPosebnez {
            name: "persons".to_string(),
            string_vec_list: persons
        }
    }
}

#[component]
pub fn InputOrganizations( organizations: OrganizationsSignal) -> Element {
    let organizations = use_signal(move || {
        organizations
            .read()
            .clone()
            .into_iter()
            .map(|(organization, aliases)| {
                (organization.name, aliases.iter().map(|alias| alias.name.clone()).collect::<Vec<_>>())
            })
            .collect::<Vec<_>>()
    });

    rsx! {
        label {
            class: "flex pb-2 font-semibold",
            "Organizacije"
        }
        SublistInputList {
            name: "organizations".to_string(),
            string_vec_list: organizations
        }
        LastInputOziromaVaskiPosebnez {
            name: "organizations".to_string(),
            string_vec_list: organizations
        }
    }
}

#[component]
pub fn InputLocations(locations: LocationsSignal) -> Element {
    let locations = use_signal(move || {
        locations
            .read()
            .clone()
            .into_iter()
            .map(|(location, aliases)| {
                (location.name, aliases.iter().map(|alias| alias.name.clone()).collect::<Vec<_>>())
            })
            .collect::<Vec<_>>()
    });

    rsx! {
        label {
            class: "flex pb-2 font-semibold",
            "Lokacije"
        }
        SublistInputList {
            name: "locations".to_string(),
            string_vec_list: locations
        }
        LastInputOziromaVaskiPosebnez {
            name: "locations".to_string(),
            string_vec_list: locations
        }
    }
}

#[component]
pub fn InputLanguages(document: DocumentSignal) -> Element {
    rsx! {
        label {
            class: "flex pb-2 font-semibold",
            "Jeziki"
        }
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
        label {
            class: "flex pb-2 font-semibold",
            "Stanje"
        }
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
