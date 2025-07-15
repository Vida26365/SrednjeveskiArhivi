use dioxus::events::Key::Enter;
use dioxus::logger::tracing::{debug, info};
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use dioxus_heroicons::IconShape;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, Iterable};
use strum::IntoEnumIterator;

use crate::database::get_database;
use crate::entities::document::{Keywords, Languages, ReviewStatus};
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
use crate::utils::date::{Calendar, Date};
use crate::utils::language::Language;

type DocumentParam = Signal<DocumentModel>;
type LocationParam = Signal<Option<LocationModel>>;
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

    if values["date"].as_value().trim() == "" {
        document.date = Set(None);
    } else {
        // TODO: Handle errors
        let calendar = Calendar::from_variant_name(&values["calendar"].as_value()).unwrap();
        let date = Date::parse(&values["date"].as_value(), &calendar).unwrap();
        document.date = Set(Some(date));
    }

    //TODO: Persons
    // TODO: Handle organizations
    // TODO: Handle locations

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
    location: LocationParam,
    locations: LocationsParam,
    organizations: OrganizationsParam,
    persons: PersonsParam,
) -> Element {
    rsx! {
        form {
            onsubmit: move |event| async move {
                submit(document.read().clone().into(), event).await;
            },
            ul {
                class: "space-y-4",
                li { InputFilename { document } }
                li { InputName { document } }
                li { InputDate { document } }
                li { InputPersons { persons } }
                li { InputOrganisations { organizations } }
                li { InputLocations { location, locations } }
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
fn InputDate(document: DocumentParam) -> Element {
    rsx! {
        label {
            class: "flex pb-2 font-semibold",
            for: "date",
            "Datum"
        }
        input {
            class: "input mb-2 w-full",
            aria_autocomplete: "none",
            autocapitalize: "false",
            autocomplete: "false",
            spellcheck: "false",
            name: "date",
            id: "date",
            value: "{document.read().date.map_or(\"\".to_string(), |date| date.to_string())}",
        }
        fieldset {
            class: "space-y-2",
            for calendar in Calendar::iter() {
                div {
                    input {
                        class: "radio",
                        type: "radio",
                        name: "calendar",
                        id: "calendar-{calendar.as_variant_name()}",
                        value: "{calendar.as_variant_name()}",
                        checked: "{document.read().date.map_or(false, |date| date.calendar() == calendar)}",
                    }
                    label {
                        class: "ps-2",
                        for: "calendar-{calendar.as_variant_name()}",
                        "{capitalize(&calendar.to_string())}"
                    }
                }
            }
        }
    }
}

#[component]
fn InputPersons(persons: PersonsParam) -> Element {
    let mut persons = use_signal(move || persons.read().clone().into_iter().map(|(person, _)| person.name).collect::<Vec<_>>());
    let mut additional = use_signal(String::new);

    rsx! {
        label {
            class: "flex pb-2 font-semibold",
            "Osebe"
        }

        for oseba in persons.read().iter().cloned() {
            div {
                class: "input w-full mb-2",
                input {
                    aria_autocomplete: "none",
                    autocapitalize: "false",
                    autocomplete: "false",
                    spellcheck: "false",
                    name: "persons",
                    value: "{oseba}",
                    oninput: {
                        let keyword = oseba.clone();
                        move |event: Event<FormData>| {
                            let mut persons = persons.write();
                            match persons.iter().position(|existing| existing == &keyword) {
                                Some(pos) => persons[pos] = event.value(),
                                None => persons.push(event.value()),
                            }
                        }
                    },
                    onkeypress: move |event| {
                        if event.key() == Enter {
                            event.prevent_default();
                        }
                    }
                }
                button {
                    class: "cursor-pointer text-base-content/50 hover:text-base-content",
                    onclick: {
                        let keyword = oseba.clone();
                        move |event: Event<MouseData>| {
                            event.prevent_default();
                            persons.write().retain(|existing| existing != &keyword);
                        }
                    },
                    svg {
                        class: "size-4 shrink-0",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        { Shape::Trash.path() }
                    }
                }
            }
        }

        div {
            class: "input w-full",
            input {
                aria_autocomplete: "none",
                autocapitalize: "false",
                autocomplete: "false",
                spellcheck: "false",
                name: "keywords",
                value: "{additional}",
                oninput: move |event| {
                    additional.set(event.value());
                },
                onkeypress: move |event| {
                    if event.key() == Enter {
                        event.prevent_default();
                        persons.write().push(additional.read().clone());
                        additional.set(String::new());
                    }
                }
            }
            button {
                class: "cursor-pointer text-base-content/50 hover:text-base-content",
                onclick: move |event| {
                    event.prevent_default();
                    additional.set(String::new());
                },
                svg {
                    class: "size-4 shrink-0",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    view_box: "0 0 24 24",
                    { Shape::Trash.path() }
                }
            }
        }
    }
}

#[component]
fn InputOrganisations(organizations: OrganizationsParam) -> Element {
    let mut organisations = use_signal(move || organizations.read().clone().into_iter().map(|(organization, _)| organization.name).collect::<Vec<_>>());
    let mut additional = use_signal(String::new);

    rsx! {
        label {
            class: "flex pb-2 font-semibold",
            "Organizacije"
        }

        for organizacija in organisations.read().iter().cloned() {
            div {
                class: "input w-full mb-2",
                input {
                    aria_autocomplete: "none",
                    autocapitalize: "false",
                    autocomplete: "false",
                    spellcheck: "false",
                    name: "organisations",
                    value: "{organizacija}",
                    oninput: {
                        let keyword = organizacija.clone();
                        move |event: Event<FormData>| {
                            let mut organisations = organisations.write();
                            match organisations.iter().position(|existing| existing == &keyword) {
                                Some(pos) => organisations[pos] = event.value(),
                                None => organisations.push(event.value()),
                            }
                        }
                    },
                    onkeypress: move |event| {
                        if event.key() == Enter {
                            event.prevent_default();
                        }
                    }
                }
                button {
                    class: "cursor-pointer text-base-content/50 hover:text-base-content",
                    onclick: {
                        let keyword = organizacija.clone();
                        move |event: Event<MouseData>| {
                            event.prevent_default();
                            organisations.write().retain(|existing| existing != &keyword);
                        }
                    },
                    svg {
                        class: "size-4 shrink-0",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        { Shape::Trash.path() }
                    }
                }
            }
        }

        div {
            class: "input w-full",
            input {
                aria_autocomplete: "none",
                autocapitalize: "false",
                autocomplete: "false",
                spellcheck: "false",
                name: "keywords",
                value: "{additional}",
                oninput: move |event| {
                    additional.set(event.value());
                },
                onkeypress: move |event| {
                    if event.key() == Enter {
                        event.prevent_default();
                        organisations.write().push(additional.read().clone());
                        additional.set(String::new());
                    }
                }
            }
            button {
                class: "cursor-pointer text-base-content/50 hover:text-base-content",
                onclick: move |event| {
                    event.prevent_default();
                    additional.set(String::new());
                },
                svg {
                    class: "size-4 shrink-0",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    view_box: "0 0 24 24",
                    { Shape::Trash.path() }
                }
            }
        }
    }
}

#[component]
fn InputLocations(location: LocationParam, locations: LocationsParam) -> Element {
    rsx! {
        label {
            class: "flex pb-2 font-semibold",
            for: "main-location",
            "Lokacija"
        }
        input {
            class: "input w-full",
            aria_autocomplete: "none",
            autocapitalize: "false",
            autocomplete: "false",
            spellcheck: "false",
            name: "main-location",
            id: "main-location",
            value: "{location.read().clone().map_or(\"\".to_string(), |location| location.name)}",
        }
        // TODO: Ostale lokacije
    }
}

#[component]
fn InputKeywords(document: DocumentParam) -> Element {
    let mut keywords = use_signal(move || document.read().keywords.0.clone());
    let mut additional = use_signal(String::new);

    rsx! {
        label {
            class: "flex pb-2 font-semibold",
            "Ključne besede"
        }

        for keyword in keywords.read().iter().cloned() {
            div {
                class: "input w-full mb-2",
                input {
                    aria_autocomplete: "none",
                    autocapitalize: "false",
                    autocomplete: "false",
                    spellcheck: "false",
                    name: "keywords",
                    value: "{keyword}",
                    oninput: {
                        let keyword = keyword.clone();
                        move |event: Event<FormData>| {
                            let mut keywords = keywords.write();
                            match keywords.iter().position(|existing| existing == &keyword) {
                                Some(pos) => keywords[pos] = event.value(),
                                None => keywords.push(event.value()),
                            }
                        }
                    },
                    onkeypress: move |event| {
                        if event.key() == Enter {
                            event.prevent_default();
                        }
                    }
                }
                button {
                    class: "cursor-pointer text-base-content/50 hover:text-base-content",
                    onclick: {
                        let keyword = keyword.clone();
                        move |event: Event<MouseData>| {
                            event.prevent_default();
                            keywords.write().retain(|existing| existing != &keyword);
                        }
                    },
                    svg {
                        class: "size-4 shrink-0",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        { Shape::Trash.path() }
                    }
                }
            }
        }

        div {
            class: "input w-full",
            input {
                aria_autocomplete: "none",
                autocapitalize: "false",
                autocomplete: "false",
                spellcheck: "false",
                name: "keywords",
                value: "{additional}",
                oninput: move |event| {
                    additional.set(event.value());
                },
                onkeypress: move |event| {
                    if event.key() == Enter {
                        event.prevent_default();
                        keywords.write().push(additional.read().clone());
                        additional.set(String::new());
                    }
                }
            }
            button {
                class: "cursor-pointer text-base-content/50 hover:text-base-content",
                onclick: move |event| {
                    event.prevent_default();
                    additional.set(String::new());
                },
                svg {
                    class: "size-4 shrink-0",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    view_box: "0 0 24 24",
                    { Shape::Trash.path() }
                }
            }
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
