use dioxus::events::Key::Enter;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use strum::IntoEnumIterator;

use crate::database::get_database;
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
use crate::utils::date::Calendar;
use crate::utils::language::Language;

type DocumentParam = Signal<DocumentModel>;
type LocationParam = Signal<Option<LocationModel>>;
type LocationsParam = Signal<Vec<(LocationModel, Vec<LocationAliasModel>)>>;
type OrganizationsParam = Signal<Vec<(OrganizationModel, Vec<OrganizationAliasModel>)>>;
type PersonsParam = Signal<Vec<(PersonModel, Vec<PersonAliasModel>)>>;

#[component]
pub fn PaneInput(
    document: DocumentParam,
    location: LocationParam,
    locations: LocationsParam,
    organizations: OrganizationsParam,
    persons: PersonsParam,
) -> Element {
    rsx! {
        form {
            onsubmit: move |event: Event<FormData>| async move {
                let mut document: DocumentActiveModel = document.read().clone().into();

                let values = event.values();
                document.title = Set(values["title"].as_value());

                let database = get_database().await;
                document.update(database).await.unwrap(); // TODO: Handle error

                info!("Submitted! {event:?}")

            },
            ul {
                li { InputFilename { document } }
                li { InputName { document } }
                li { InputDate { document } }
                li { InputPersons { persons } }
                li { InputOrganisations { organizations } }
                li { InputLocations { location, locations } }
                li { InputKeywords { document } }
                li { InputLanguages { document } }
                li {
                    button { "Shrani" }
                }
            }
        }
    }
}

#[component]
fn InputFilename(document: DocumentParam) -> Element {
    rsx! {
        label { "Ime datoteke: " }
        label { "{document.read().filename}" }
    }
}

#[component]
fn InputName(document: DocumentParam) -> Element {
    rsx! {
        label { "Naslov: " }
        input { name : "title", value: "{document.read().title}" }
    }
}

#[component]
fn InputDate(document: DocumentParam) -> Element {
    rsx! {
        label { "Datum: " }
        input {
            name: "date",
            value: "{document.read().date.map_or(\"\".to_string(), |date| date.to_string())}",
        }
        select {
            name: "calendar",
            for calendar in Calendar::iter() {
                option {
                    value: "{calendar.as_variant_name()}",
                    "{calendar}"
                }
            }
        }
    }
}

#[component]
fn InputPersons(persons: PersonsParam) -> Element {
    rsx! {
        label { "Osebe: " }
        label { "TODO" }
    }
}

#[component]
fn InputOrganisations(organizations: OrganizationsParam) -> Element {
    rsx! {
        label { "Organizacije: " }
        label { "TODO" }
    }
}

#[component]
fn InputLocations(location: LocationParam, locations: LocationsParam) -> Element {
    rsx! {
        label { "Lokacija: " }
        input {
            name: "main-location",
            value: "{location.read().clone().map_or(\"\".to_string(), |location| location.name)}"
        }
        // TODO: Ostale lokacije
    }
}

#[component]
fn InputKeywords(document: DocumentParam) -> Element {
    // TODO: Make this work

    let mut signal = use_signal(move || document.read().keywords.0.clone());

    rsx! {
        label {"Ključne besede: " }

        input {
            name: "new-keyword",
            value: "",
            onkeypress: move |event: Event<KeyboardData>| {
                if event.key() == Enter {
                    // event.prevent_default();
                    signal.write().push(String::new());
                    println!("Enter pressed {:?}", &event);
                    println!("Keywords: {:?}", signal);
                }
            }
        }

        for klb in signal.read().clone() { // TODO: Lepša rešitev za vse
            input {
                name: "keywords",
                value: "{klb}",
                onkeypress: move |event: Event<KeyboardData>| {
                    if event.key() == Enter {
                        // event.prevent_default();
                        signal.write().push(String::new());
                        println!("Enter pressed {:?}", &event);
                        println!("Keywords: {:?}", signal);
                    }
                }
            }
        }
    }
}

#[component]
fn InputLanguages(document: DocumentParam) -> Element {
    rsx! {
        label { "Jeziki:" }
        ul {
            padding_left: "10px",
            for language in Language::iter() {
                li {
                    input {
                        type: "checkbox",
                        name: "language",
                        value: "{language.as_two_letter_code()}",
                    }
                    label { "{language.as_name()}" }
                }
            }
        }
    }
}
