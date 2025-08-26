use anyhow::{Context, Result};
use dioxus::logger::tracing::{debug, info};
use dioxus::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::components::alerts::{AlertError, AlertSuccess};
use crate::components::documents::display::{
    DocumentSignal,
    LocationsSignal,
    OrganizationsSignal,
    PersonsSignal,
};
use crate::database::get_database;
use crate::entities::document::{Keywords, Languages, ReviewStatus};
use crate::entities::{
    DocumentActiveModel,
    DocumentLocation,
    DocumentLocationActiveModel,
    DocumentLocationColumn,
    DocumentModel,
    DocumentOrganization,
    DocumentOrganizationActiveModel,
    DocumentOrganizationColumn,
    DocumentPerson,
    DocumentPersonActiveModel,
    DocumentPersonColumn,
    LocationActiveModel,
    LocationAliasActiveModel,
    OrganizationActiveModel,
    OrganizationAliasActiveModel,
    PersonActiveModel,
    PersonAliasActiveModel,
};
use crate::utils::language::Language;

mod basic;
mod keywords;
mod list_inputov_generator;

use basic::{
    InputFilename,
    InputLanguages,
    InputLocations,
    InputName,
    InputOrganizations,
    InputPersons,
    InputReview,
};
use keywords::InputKeywords;

async fn submit(document: &DocumentModel, event: Event<FormData>) -> Result<()> {
    debug!("Event: {event:#?}");

    let database = get_database().await;
    let values = event.values();

    let mut modified: DocumentActiveModel = document.clone().into();

    modified.title = Set(values["title"].as_value());

    // Delete old document <-> entity mappings
    // TODO: Keep existing ones in the future
    DocumentPerson::delete_many()
        .filter(DocumentPersonColumn::Document.eq(document.id))
        .exec(database)
        .await?;
    DocumentOrganization::delete_many()
        .filter(DocumentOrganizationColumn::Document.eq(document.id))
        .exec(database)
        .await?;
    DocumentLocation::delete_many()
        .filter(DocumentLocationColumn::Document.eq(document.id))
        .exec(database)
        .await?;

    match values.get("persons") {
        Some(osebe) => {
            for oseba in osebe.as_slice().iter().filter(|o| !o.trim().is_empty()) {
                let person = PersonActiveModel {
                    id: Set(Uuid::now_v7()),
                    name: Set(oseba.trim().to_string()),
                    description: Set(String::new()),
                };
                let person = person.insert(database).await.context("Failed to insert person")?;
                let document_person = DocumentPersonActiveModel {
                    document: Set(document.id),
                    person: Set(person.id),
                };
                document_person.insert(database).await.context("Failed to insert person link")?;
                match values.get(&format!("persons/{oseba}")) {
                    Some(variacije) => {
                        for variacija in
                            variacije.as_slice().iter().filter(|o| !o.trim().is_empty())
                        {
                            let alias = PersonAliasActiveModel {
                                id: Set(Uuid::now_v7()),
                                person: Set(Some(person.id)),
                                name: Set(variacija.trim().to_string()),
                                description: Set(String::new()),
                            };
                            alias
                                .insert(database)
                                .await
                                .context("Failed to insert person alias")?;
                        }
                    }
                    None => {}
                }
            }
        }
        None => {}
    }

    match values.get("organizations") {
        Some(organizacije) => {
            for organizacija in organizacije.as_slice().iter().filter(|o| !o.trim().is_empty()) {
                let org_model = OrganizationActiveModel {
                    id: Set(Uuid::now_v7()),
                    name: Set(organizacija.trim().to_string()),
                    description: Set(String::new()),
                };
                let organization =
                    org_model.insert(database).await.context("Failed to insert organization")?;
                let document_organization = DocumentOrganizationActiveModel {
                    document: Set(document.id),
                    organization: Set(organization.id),
                };
                document_organization
                    .insert(database)
                    .await
                    .context("Failed to insert organization link")?;
                match values.get(&format!("organizations/{organizacija}")) {
                    Some(variacije) => {
                        for variacija in
                            variacije.as_slice().iter().filter(|o| !o.trim().is_empty())
                        {
                            let alias = OrganizationAliasActiveModel {
                                id: Set(Uuid::now_v7()),
                                organization: Set(Some(organization.id)),
                                name: Set(variacija.trim().to_string()),
                                description: Set(String::new()),
                            };
                            alias
                                .insert(database)
                                .await
                                .context("Failed to insert organization alias")?;
                        }
                    }
                    None => {}
                }
            }
        }
        None => {}
    }

    match values.get("locations") {
        Some(lokacije) => {
            for lokacija in lokacije.as_slice().iter().filter(|o| !o.trim().is_empty()) {
                let location = LocationActiveModel {
                    id: Set(Uuid::now_v7()),
                    name: Set(lokacija.trim().to_string()),
                    description: Set(String::new()),
                };
                let location =
                    location.insert(database).await.context("Failed to insert location")?;
                let document_location = DocumentLocationActiveModel {
                    document: Set(document.id),
                    location: Set(location.id),
                };
                document_location
                    .insert(database)
                    .await
                    .context("Failed to insert location link")?;
                match values.get(&format!("locations/{lokacija}")) {
                    Some(variacije) => {
                        for variacija in
                            variacije.as_slice().iter().filter(|o| !o.trim().is_empty())
                        {
                            let alias = LocationAliasActiveModel {
                                id: Set(Uuid::now_v7()),
                                location: Set(Some(location.id)),
                                name: Set(variacija.trim().to_string()),
                                description: Set(String::new()),
                            };
                            alias
                                .insert(database)
                                .await
                                .context("Failed to insert location alias")?;
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
            modified.keywords = Set(Keywords(
                keywords
                    .as_slice()
                    .iter()
                    .map(|kw| kw.trim())
                    .filter(|kw| !kw.is_empty())
                    .map(String::from)
                    .collect(),
            ))
        }
        None => modified.keywords = Set(Keywords(vec![])),
    }

    match values.get("languages") {
        Some(languages) => {
            modified.languages = Set(Languages(
                languages
                    .as_slice()
                    .iter()
                    .filter_map(|lang| Language::from_two_letter_code(lang))
                    .collect(),
            ))
        }
        None => modified.languages = Set(Languages(vec![])),
    }

    match ReviewStatus::from_variant_name(&values["review"].as_value()) {
        Some(review) => modified.review = Set(review),
        None => modified.review = Set(ReviewStatus::NotReviewed),
    }

    debug!("Parsed: {modified:?}");

    modified.update(database).await.context("Failed to update document")?;

    info!("Submitted!");

    Ok(())
}

#[component]
pub fn PaneProperties(
    #[props(into)] document: DocumentSignal,
    #[props(into)] locations: LocationsSignal,
    #[props(into)] organizations: OrganizationsSignal,
    #[props(into)] persons: PersonsSignal,
    refresh: Callback<()>,
) -> Element {
    let mut state: Signal<Option<Element>> = use_signal(|| None);

    rsx! {
        form {
            onsubmit: move |event| async move {
                match submit(&document.read(), event).await {
                    Ok(_) => state.set(Some(rsx! {
                        AlertSuccess {
                            title: "Dokument uspešno shranjen",
                        }
                    })),
                    Err(error) => state.set(Some(rsx! {
                        AlertError {
                            title: "Napaka pri shranjevanju dokumenta",
                            details: format!("{error:?}"),
                        }
                    })),
                }
                refresh.call(());
            },
            ul {
                class: "space-y-4 pb-1 min-w-fit",
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
                        type: "submit",
                        "Shrani"
                    }
                }
                match state() {
                    Some(element) => rsx! { li { { element } } },
                    None => rsx! {},
                }
            }
        }
    }
}
