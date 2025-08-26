use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result};
use dioxus::html::FileEngine;
use dioxus::logger::tracing::error;
use dioxus::prelude::*;
use dioxus_heroicons::IconShape;
use dioxus_heroicons::outline::Shape;
use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelTrait,
    EntityTrait,
    FromQueryResult,
    IntoActiveModel,
    QuerySelect,
    Set,
    TransactionTrait,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::components::alerts::{AlertError, AlertSuccess, AlertWarning};
use crate::components::files::FileUpload;
use crate::components::skeleton::Skeleton;
use crate::database::get_database;
use crate::entities::{Document, DocumentColumn};

#[derive(Clone, Debug, PartialEq, Eq)]
enum ImportState {
    Idle,
    Success,
    Error(String),
}

#[derive(Clone, Debug, PartialEq, Eq, FromQueryResult)]
struct DocumentInfo {
    id: Uuid,
    title: String,
    has_metadata: bool,
    has_text: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
struct DocumentSegmented {
    summary: String,
    metadata: String,
    content: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
struct DocumentMetadata {
    date: Option<String>,
    location: Option<String>,
    languages: Vec<String>,
    keywords: Vec<String>,
}

#[component]
pub fn OcrExternalImport() -> Element {
    let mut documents: Resource<Result<HashMap<Uuid, DocumentInfo>>> =
        use_resource(move || async move {
            let database = get_database().await;

            let query = Document::find()
                .select_only()
                .column(DocumentColumn::Id)
                .column(DocumentColumn::Title)
                .expr_as(
                    Expr::col(DocumentColumn::Date)
                        .is_not_null()
                        .or(Expr::col(DocumentColumn::Location).is_not_null())
                        .or(Expr::col(DocumentColumn::Languages).ne("[]"))
                        .or(Expr::col(DocumentColumn::Keywords).ne("[]")),
                    "has_metadata",
                )
                .expr_as(
                    Expr::col(DocumentColumn::Summary)
                        .ne("")
                        .or(Expr::col(DocumentColumn::Metadata).ne(""))
                        .or(Expr::col(DocumentColumn::Content).ne("")),
                    "has_text",
                )
                .into_model::<DocumentInfo>();

            let documents = query
                .all(database)
                .await?
                .into_iter()
                .map(|document| (document.id, document))
                .collect();

            Ok(documents)
        });

    let refresh = move || {
        documents.restart();
    };

    rsx! {
        match &*documents.read_unchecked() {
            Some(Ok(documents)) => rsx! {
                OcrExternalImportInner { documents: documents.clone(), refresh }
            },
            Some(Err(error)) => rsx! {
                AlertError {
                    title: "Napaka pri nalaganju dokumentov",
                    details: format!("{error:?}"),
                }
            },
            None => rsx! {
                Skeleton {}
            },
        }
    }
}

#[component]
fn OcrExternalImportInner(
    documents: ReadOnlySignal<HashMap<Uuid, DocumentInfo>>,
    refresh: Callback<()>,
) -> Element {
    let mut state = use_signal(|| ImportState::Idle);

    let mut segmented = use_signal(HashMap::<Uuid, PathBuf>::new);
    let mut metadata = use_signal(HashMap::<Uuid, PathBuf>::new);

    let upload = move |engine: Arc<dyn FileEngine>| {
        for path in engine.files() {
            let path = PathBuf::from(path);

            // TODO: Add support for ZIP archives?
            // TODO: Move parsing code to here and handle errors?

            let Some(filename) = path.file_name().and_then(|str| str.to_str()) else {
                error!("Invalid file name: {path:?}");
                continue;
            };

            let parts: Vec<&str> = filename.split('.').collect();
            if parts.len() != 3 {
                error!("Invalid file name: {filename}");
                continue;
            }

            let (uuid, kind, ext) = (parts[0], parts[1], parts[2]);
            if ext != "json" {
                error!("Invalid file extension: {filename}");
                continue;
            }

            let Ok(uuid) = Uuid::parse_str(uuid) else {
                error!("Invalid file UUID: {filename}");
                continue;
            };

            if !path.is_file() {
                error!("File does not exist: {}", path.display());
                continue;
            }

            let documents = &*documents.read();

            if !documents.contains_key(&uuid) {
                error!("Document does not exist: {filename}");
                continue;
            }

            match kind {
                "segmented" => {
                    segmented.write().insert(uuid, path);
                }
                "metadata" => {
                    metadata.write().insert(uuid, path);
                }
                _ => {
                    error!("Invalid file type: {filename}");
                    continue;
                }
            }

            state.set(ImportState::Idle);
        }
    };

    let remove_segmented = move |id: Uuid| {
        segmented.write().remove(&id);
        state.set(ImportState::Idle);
    };

    let remove_metadata = move |id: Uuid| {
        metadata.write().remove(&id);
        state.set(ImportState::Idle);
    };

    let mut save = async move || -> Result<()> {
        let database = get_database().await;
        let txn = database.begin().await?;

        for (id, path) in segmented.read().iter() {
            let mut document = Document::find_by_id(*id)
                .one(&txn)
                .await
                .with_context(|| format!("Failed to find document: {id}"))?
                .with_context(|| format!("Failed to find document: {id}"))?
                .into_active_model();

            let content = tokio::fs::read_to_string(path)
                .await
                .with_context(|| format!("Failed to read file: {}", path.display()))?;

            let data: DocumentSegmented = serde_json::from_str(&content)
                .with_context(|| format!("Failed to parse file: {}", path.display()))?;

            document.summary = Set(data.summary);
            document.metadata = Set(data.metadata);
            document.content = Set(data.content);

            document
                .update(&txn)
                .await
                .with_context(|| format!("Failed to update document: {id}"))?;
        }

        // TODO: Import metadata

        txn.commit().await?;

        // Clear pending files
        segmented.write().clear();
        metadata.write().clear();

        // Reload the document list
        refresh.call(());

        Ok(())
    };

    rsx! {
        div {
            class: "space-y-4 pb-1 min-w-min",

            FileUpload {
                handler: upload,
                accept: ".segmented.json,.metadata.json",
                multiple: true,
            }

            button {
                class: "btn btn-soft btn-primary w-full mb-4 rounded-box",
                type: "submit",
                disabled: segmented.read().is_empty() && metadata.read().is_empty(),
                onclick: move |_| async move {
                    match save().await {
                        Ok(_) => state.set(ImportState::Success),
                        Err(error) => state.set(ImportState::Error(format!("{error:?}"))),
                    }
                },
                "Uvozi vsebino"
            }

            match state() {
                ImportState::Idle => rsx! {
                    AlertWarning {
                        title: "Uvoz prepiše obstoječo besedilo in metapodatke",
                    }
                },
                ImportState::Success => rsx! {
                    AlertSuccess {
                        title: "Vsebina uspešno uvožena",
                    }
                },
                ImportState::Error(error) => rsx! {
                    AlertError {
                        title: "Napaka pri uvažanju vsebine",
                        details: error,
                    }
                },
            }

            ImportedList {
                heading: "Obdelano besedilo",
                entries: segmented,
                documents: documents,
                warn_text: "Besedilo že obstaja",
                warn_predicate: |document: &DocumentInfo| document.has_text,
                on_remove: remove_segmented,
            }

            ImportedList {
                heading: "Obdelani metapodatki",
                entries: metadata,
                documents: documents,
                warn_text: "Metapodatki že obstajajo",
                warn_predicate: |document: &DocumentInfo| document.has_metadata,
                on_remove: remove_metadata,
            }
        }
    }
}

#[component]
fn ImportedList(
    heading: &'static str,
    entries: ReadOnlySignal<HashMap<Uuid, PathBuf>>,
    documents: ReadOnlySignal<HashMap<Uuid, DocumentInfo>>,
    warn_text: &'static str,
    warn_predicate: fn(&DocumentInfo) -> bool,
    on_remove: Callback<Uuid>,
) -> Element {
    let entries = &*entries.read();
    let documents = &*documents.read();

    let mut documents = entries.iter().filter_map(|(id, _)| documents.get(id).cloned()).peekable();

    rsx! {
        if documents.peek().is_some() {
            div {
                class: "space-y-2",

                div {
                    class: "p-3 rounded-box bg-alt-100",
                    span { class: "text-lg font-semibold", "{heading}" }
                }

                for document in documents {
                    div {
                        key: document.id,
                        class: "flex justify-between gap-3 p-3 rounded-box bg-alt-100",
                        p {
                            class: "flex gap-2",
                            span {
                                class: "text-sm font-semibold truncate max-w-75",
                                "{document.title}"
                            }
                            if warn_predicate(&document) {
                                span {
                                    class: "badge badge-soft badge-warning whitespace-nowrap text-xs",
                                    "{warn_text}"
                                }
                            }
                        }
                        button {
                            class: "cursor-pointer text-base-content/50 hover:text-base-content",
                            type: "button",
                            onclick: move |_| { on_remove.call(document.id); },
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
        }
    }
}
