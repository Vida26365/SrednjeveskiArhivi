use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result};
use dioxus::html::FileEngine;
use dioxus::logger::tracing::error;
use dioxus::prelude::*;
use dioxus_heroicons::IconShape;
use dioxus_heroicons::outline::Shape;
use sea_orm::entity::prelude::*;
use sea_orm::{Set, TransactionTrait};

use crate::components::alerts::{AlertError, AlertSuccess};
use crate::components::files::FileUpload;
use crate::database::get_database;
use crate::directories::DIRECTORIES;
use crate::entities::document;

#[derive(Clone, Debug, PartialEq, Eq)]
enum UploadState {
    Idle,
    Success,
    Error(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct FileDetails {
    path: PathBuf,
    name: String,
    size: String,
}

fn filesize(size: u64) -> String {
    let mut size = size as f64;
    let mut unit = "B";

    let units = ["B", "KiB", "MiB", "GiB", "TiB"];

    for &current in units.iter().skip(1) {
        if size < 1024.0 {
            break;
        }
        size /= 1024.0;
        unit = current;
    }

    format!("{size:.2} {unit}")
}

#[component]
pub fn DocumentUpload() -> Element {
    let mut state = use_signal(|| UploadState::Idle);

    let mut uploaded = use_signal(Vec::<FileDetails>::new);

    let upload = move |engine: Arc<dyn FileEngine>| {
        for path in engine.files() {
            let path = PathBuf::from(path);

            if path.extension().and_then(|ext| ext.to_str()) != Some("pdf") {
                error!("Invalid file type: {}", path.display());
                continue;
            }

            if !path.is_file() {
                error!("File does not exist: {}", path.display());
                continue;
            }

            if uploaded.read().iter().any(|existing| existing.path == path) {
                error!("File already uploaded: {}", path.display());
                continue;
            }

            let file = FileDetails {
                path: path.clone(),
                name: path.file_name().unwrap_or("Neznano".as_ref()).to_string_lossy().to_string(),
                size: filesize(path.metadata().map(|metadata| metadata.len()).unwrap_or(0)),
            };

            uploaded.write().push(file);
            state.set(UploadState::Idle);
        }
    };

    let mut remove = move |file: &FileDetails| {
        uploaded.write().retain(|existing| existing.path != file.path);
        state.set(UploadState::Idle);
    };

    let save = async move || -> Result<()> {
        let storage = DIRECTORIES.userdata.join("documents");
        tokio::fs::create_dir_all(&storage).await.context("Failed to create storage")?;

        let database = get_database().await;
        let txn = database.begin().await?;

        for file in uploaded.read().iter() {
            let id = Uuid::now_v7();

            let document = document::ActiveModel {
                id: Set(id),
                filename: Set(file.name.clone()),
                title: Set(file.name.trim_end_matches(".pdf").to_string()),
                ..Default::default()
            };

            let path = storage.join(id.to_string()).with_extension("pdf");

            tokio::fs::copy(&file.path, &path)
                .await
                .with_context(|| format!("Failed to copy file: {}", file.path.display()))?;

            document
                .insert(&txn)
                .await
                .with_context(|| format!("Failed to insert document: {}", file.path.display()))?;
        }

        txn.commit().await?;

        Ok(())
    };

    rsx! {
        div {
            class: "space-y-4 pb-1 min-w-min",

            FileUpload {
                handler: upload,
                accept: ".pdf",
                multiple: true,
            }

            button {
                class: "btn btn-soft btn-primary w-full rounded-box",
                disabled: uploaded.read().is_empty(),
                onclick: move |_| async move {
                    match save().await {
                        Ok(_) => {
                            uploaded.write().clear();
                            state.set(UploadState::Success);
                        }
                        Err(error) => {
                            state.set(UploadState::Error(format!("{error:?}")));
                        }
                    }
                },
                "Dodaj dokumente"
            }

            match state() {
                UploadState::Idle => rsx! {},
                UploadState::Success => rsx! {
                    AlertSuccess {
                        title: "Dokumenti uspešno dodani",
                    }
                },
                UploadState::Error(error) => rsx! {
                    AlertError {
                        title: "Napaka pri dodajanju dokumentov",
                        details: error,
                    }
                },
            }

            div {
                class: "space-y-2",
                for file in uploaded.read().iter().cloned().rev() {
                    div {
                        class: "flex justify-between gap-3 p-3 rounded-box bg-alt-100",
                        div {
                            class: "flex",
                            div {
                                p {
                                    class: "text-sm font-semibold inline-block truncate max-w-75",
                                    "{file.name}"
                                }
                                p {
                                    class: "text-xs text-base-content/50",
                                    "{file.size}"
                                }
                            }
                        }
                        button {
                            class: "cursor-pointer text-base-content/50 hover:text-base-content",
                            onclick: move |_| { remove(&file); },
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
