use anyhow::{Context, Result};
use dioxus::logger::tracing::{debug, info};
use dioxus::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

// Import alert components for success/error messages
use crate::components::alerts::{AlertError, AlertSuccess};
// Import signals for document and related entities
use crate::components::documents::display::{
    DocumentSignal,
    LocationsSignal,
    OrganizationsSignal,
    PersonsSignal,
};
// Database and entity imports
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

// Input components for the properties pane
mod inputs;

use inputs::{
    InputId,
    InputKeywords,
    InputLanguages,
    InputLocations,
    InputName,
    InputOrganizations,
    InputPersons,
    InputReview,
};

/// Handles form submission for updating a document and its related entities.
/// - Updates the document's title, keywords, languages, and review status.
/// - Deletes and recreates links to persons, organizations, and locations.
/// - Inserts aliases for each entity if provided.
/// - Shows success or error alerts based on the result.
async fn submit(document: &DocumentModel, event: Event<FormData>) -> Result<()> {
    debug!("Event: {event:#?}");

    let database = get_database().await;
    let values = event.values();

    // Prepare the modified document model
    let mut modified: DocumentActiveModel = document.clone().into();

    // Update the document title
    modified.title = Set(values["title"].as_value());

    // Remove old links to persons, organizations, and locations
    // TODO: Optimize to keep existing links in the future
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

    // Add new persons and their aliases
    match values.get("persons") {
        Some(osebe) => {
            for oseba in osebe.as_slice().iter().filter(|o| !o.trim().is_empty()) {
                // Insert person
                let person = PersonActiveModel {
                    id: Set(Uuid::now_v7()),
                    name: Set(oseba.trim().to_string()),
                    description: Set(String::new()),
                };
                let person = person.insert(database).await.context("Failed to insert person")?;
                // Link person to document
                let document_person = DocumentPersonActiveModel {
                    document: Set(document.id),
                    person: Set(person.id),
                };
                document_person.insert(database).await.context("Failed to insert person link")?;
                // Insert aliases for person
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

    // Add new organizations and their aliases
    match values.get("organizations") {
        Some(organizacije) => {
            // for (org_str, index) in organizacije.as_slice().iter().enumerate() {

            // }

            for organizacija in organizacije.as_slice().iter().filter(|o| !o.trim().is_empty()) {
                // Insert organization
                let org_model = OrganizationActiveModel {
                    id: Set(Uuid::now_v7()),
                    name: Set(organizacija.trim().to_string()),
                    description: Set(String::new()),
                };
                let organization =
                    org_model.insert(database).await.context("Failed to insert organization")?;
                // Link organization to document
                let document_organization = DocumentOrganizationActiveModel {
                    document: Set(document.id),
                    organization: Set(organization.id),
                };
                document_organization
                    .insert(database)
                    .await
                    .context("Failed to insert organization link")?;
                // Insert aliases for organization
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

    // Add new locations and their aliases
    match values.get("locations") {
        Some(lokacije) => {
            for lokacija in lokacije.as_slice().iter().filter(|o| !o.trim().is_empty()) {
                // Insert location
                let location = LocationActiveModel {
                    id: Set(Uuid::now_v7()),
                    name: Set(lokacija.trim().to_string()),
                    description: Set(String::new()),
                };
                let location =
                    location.insert(database).await.context("Failed to insert location")?;
                // Link location to document
                let document_location = DocumentLocationActiveModel {
                    document: Set(document.id),
                    location: Set(location.id),
                };
                document_location
                    .insert(database)
                    .await
                    .context("Failed to insert location link")?;
                // Insert aliases for location
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

    // Update keywords for the document
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

    // Update languages for the document
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

    // Update review status for the document
    match ReviewStatus::from_variant_name(&values["review"].as_value()) {
        Some(review) => modified.review = Set(review),
        None => modified.review = Set(ReviewStatus::NotReviewed),
    }

    debug!("Parsed: {modified:?}");

    // Save changes to the database
    modified.update(database).await.context("Failed to update document")?;

    info!("Submitted!");

    Ok(())
}

/// PaneProperties renders the document properties editing form.
/// - Displays input fields for all document properties.
/// - Handles form submission and shows alerts for success or error.
/// - Includes a "Shrani" (Save) button to submit changes.
#[component]
pub fn PaneProperties(
    #[props(into)] document: DocumentSignal,
    #[props(into)] locations: LocationsSignal,
    #[props(into)] organizations: OrganizationsSignal,
    #[props(into)] persons: PersonsSignal,
    refresh: Callback<()>,
) -> Element {
    // State for showing alert messages after submission
    let mut state: Signal<Option<Element>> = use_signal(|| None);

    rsx! {
        form {
            // Handle form submission asynchronously
            onsubmit: move |event| async move {
                match submit(&document.read(), event).await {
                    Ok(_) => state.set(Some(rsx! {
                        AlertSuccess {
                            title: "Dokument uspešno shranjen", // Document successfully saved
                        }
                    })),
                    Err(error) => state.set(Some(rsx! {
                        AlertError {
                            title: "Napaka pri shranjevanju dokumenta", // Error saving document
                            details: format!("{error:?}"),
                        }
                    })),
                }
                // Refresh parent view after submission
                refresh.call(());
            },
            ul {
                class: "space-y-4 [&>li]:space-y-2 pb-1 min-w-fit",
                // Render input fields for each document property
                li { InputId { document } }
                li { InputName { document } }
                li { InputPersons { document, persons } }
                li { InputOrganizations { organizations } }
                li { InputLocations { locations } }
                li { InputKeywords { document } }
                li { InputLanguages { document } }
                li { InputReview { document } }
                // Save button for submitting the form
                li {
                    button {
                        class: "btn btn-soft btn-primary rounded-box",
                        type: "submit",
                        "Shrani"
                    }
                }
                // Show alert message if present
                match state() {
                    Some(element) => rsx! { li { { element } } },
                    None => rsx! {},
                }
            }
        }
    }
}
