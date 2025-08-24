use anyhow::{Context, Result};
use dioxus::logger::tracing::{debug, info};
use dioxus::prelude::*;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;

use crate::components::alerts::{AlertError, AlertInfo, AlertSuccess};
use crate::components::documents::display::{DocumentSignal, PrimaryLocationSignal};
use crate::database::get_database;
use crate::directories::DIRECTORIES;
use crate::entities::{DocumentActiveModel, DocumentModel};
use crate::utils::date::{Calendar, Date};
use crate::utils::ocr;

mod date;
mod location;

use date::InputDate;
use location::InputLocation;

async fn submit(document: &DocumentModel, event: Event<FormData>) -> Result<()> {
    debug!("Event: {event:?}");

    let database = get_database().await;
    let values = event.values();

    let mut modified: DocumentActiveModel = document.clone().into();

    modified.summary = Set(values["summary"].as_value());
    modified.metadata = Set(values["metadata"].as_value());
    modified.content = Set(values["content"].as_value());

    if values["date"].as_value().trim() == "" {
        modified.date = Set(None);
    } else {
        let calendar = &values["calendar"].as_value();
        let calendar = Calendar::from_variant_name(calendar)
            .with_context(|| format!("Invalid calendar variant: {calendar}"))?;
        let date = &values["date"].as_value();
        let date = Date::parse(date, &calendar)
            .with_context(|| format!("Invalid document date string: {date}"))?;
        modified.date = Set(Some(date));
    }

    // TODO: Handle location

    debug!("Parsed: {modified:?}");

    modified.update(database).await.context("Failed to update document")?;

    info!("Submitted!");

    Ok(())
}

async fn extract(document: &DocumentModel) -> Result<()> {
    let database = get_database().await;

    // TODO: Read languages from somewhere
    let languages = vec!["lat", "slv"];

    let storage = DIRECTORIES.userdata.join("documents");
    let path = storage.join(document.id.to_string()).with_extension("pdf");

    let text = tokio::task::spawn_blocking(move || ocr::extract(&path, &languages))
        .await
        .context("Failed to spawn blocking task")?
        .context("Failed to extract text")?;

    let mut modified: DocumentActiveModel = document.clone().into();
    modified.summary = Set("".to_string());
    modified.metadata = Set("".to_string());
    modified.content = Set(text);

    modified.update(database).await.context("Failed to update document")?;

    info!("Extracted!");

    Ok(())
}

#[component]
pub fn PaneText(
    #[props(into)] document: DocumentSignal,
    #[props(into)] location: PrimaryLocationSignal,
    refresh: Callback<()>,
) -> Element {
    let mut state: Signal<Option<Element>> = use_signal(|| None);

    rsx! {
        link { rel: "stylesheet", href: asset!("/assets/styles/autoresize.css") }
        script { src: asset!("/assets/scripts/autoresize.js") }

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
                li {
                    class: "flex gap-2",
                    InputLocation { location }
                    InputDate { document }
                }
                li {
                    label {
                        class: "flex pb-2 font-semibold",
                        for: "summary",
                        "Povzetek"
                    }
                    textarea {
                        class: "textarea autoresize w-full",
                        aria_autocomplete: "none",
                        autocapitalize: "false",
                        autocomplete: "false",
                        spellcheck: "false",
                        name: "summary",
                        id: "summary",
                        value: "{document.read().summary}",
                    }
                }
                li {
                    label {
                        class: "flex pb-2 font-semibold",
                        for: "metadata",
                        "Metapodatki"
                    }
                    textarea {
                        class: "textarea autoresize w-full",
                        aria_autocomplete: "none",
                        autocapitalize: "false",
                        autocomplete: "false",
                        spellcheck: "false",
                        name: "metadata",
                        id: "metadata",
                        value: "{document.read().metadata}",
                    }
                }
                li {
                    label {
                        class: "flex pb-2 font-semibold",
                        for: "content",
                        "Vsebina"
                    }
                    textarea {
                        class: "textarea autoresize w-full",
                        aria_autocomplete: "none",
                        autocapitalize: "false",
                        autocomplete: "false",
                        spellcheck: "false",
                        name: "content",
                        id: "content",
                        value: "{document.read().content}",
                    }
                }
                li {
                    class: "flex gap-2",
                    button {
                        class: "btn btn-soft btn-primary rounded-box",
                        type: "submit",
                        "Shrani"
                    }
                    button {
                        class: "btn btn-soft btn-accent rounded-box",
                        type: "button",
                        onclick: move |_| async move {
                            // TODO: Have confirmation dialog before running?
                            state.set(Some(rsx! {
                                AlertInfo {
                                    title: "Poteka prepoznavanje besedila ..."
                                }
                            }));
                            match extract(&document.read()).await {
                                Ok(_) => state.set(Some(rsx! {
                                    AlertSuccess {
                                        title: "Besedilo uspešno prepoznano",
                                    }
                                })),
                                Err(error) => state.set(Some(rsx! {
                                    AlertError {
                                        title: "Napaka pri prepoznavanju besedila",
                                        details: format!("{error:?}"),
                                    }
                                })),
                            }
                            refresh.call(());
                        },
                        "OCR"
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
