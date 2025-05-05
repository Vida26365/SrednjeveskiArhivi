use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result};
use dioxus::html::{FileEngine, HasFileData};
use dioxus::logger::tracing::error;
use dioxus::prelude::*;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;

use crate::components::alerts::error::AlertError;
use crate::components::alerts::success::AlertSuccess;
use crate::database::get_database;
use crate::directories::DIRECTORIES;
use crate::entities::document;
use crate::views::documents::upload::UploadState::Idle;

#[derive(Clone, Debug, PartialEq, Eq)]
struct FileDetails {
    path: PathBuf,
    name: String,
    size: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum UploadState {
    Idle,
    Success,
    Error(String),
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

    format!("{:.2} {}", size, unit)
}

#[component]
pub fn DocumentUpload() -> Element {
    let mut uploaded = use_signal(Vec::<FileDetails>::new);

    let mut state = use_signal(|| Idle);

    let upload = move |engine: Arc<dyn FileEngine>| {
        for path in engine.files() {
            let path = PathBuf::from(path);

            if path.extension().and_then(|ext| ext.to_str()) != Some("pdf") {
                error!("Invalid file type: {}", path.display());
                continue;
            }

            if !path.exists() {
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
            state.set(Idle);
        }
    };

    let mut remove = move |file: &FileDetails| {
        uploaded.write().retain(|existing| existing.path != file.path);
    };

    let save = async move || -> Result<()> {
        for file in uploaded.read().iter() {
            let id = Uuid::now_v7();

            let document = document::ActiveModel {
                id: ActiveValue::Set(id),
                filename: ActiveValue::Set(file.name.clone()),
                title: ActiveValue::Set(file.name.clone()),
                keywords: ActiveValue::Set(document::Keywords(vec![])),
                summary: ActiveValue::NotSet,
                content: ActiveValue::NotSet,
            };

            let storage = DIRECTORIES.userdata.join("documents");
            let path = storage.join(id.to_string()).with_extension("pdf");

            tokio::fs::create_dir_all(&storage).await.context("Failed to create storage")?;
            tokio::fs::copy(&file.path, &path).await.context("Failed to copy file")?;

            let database = get_database().await;
            document.insert(database).await.context("Failed to insert document")?;
        }

        Ok(())
    };

    let mut hovered = use_signal(|| false);
    let background = use_memo(move || if hovered() { "bg-alt-300" } else { "bg-alt-100" });

    rsx! {
        label {
            class: "flex flex-col items-center justify-center w-full h-64 mb-4 rounded-box cursor-pointer {background} hover:bg-alt-200",
            for: "upload",
            ondragover: move |evt| { evt.prevent_default(); hovered.set(true); },
            ondragleave: move |evt| { evt.prevent_default(); hovered.set(false); },
            ondrop: move |evt| { evt.prevent_default(); hovered.set(false); evt.files().map(upload); },
            svg {
                class: "mb-4",
                width: "48",
                height: "48",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                view_box: "0 0 24 24",
                path { d: "M3 16.5v2.25A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75V16.5m-13.5-9L12 3m0 0 4.5 4.5M12 3v13.5" }
            }
            p {
                class: "mb-2 text-sm",
                span { class: "font-semibold", "Kliknite za nalaganje" }
                " ali povlecite in spustite"
            }
            input {
                type: "file",
                id: "upload",
                class: "hidden",
                accept: ".pdf",
                multiple: true,
                onchange: move |evt| { evt.files().map(upload); }
            }
        }

        button {
            class: "btn btn-soft btn-primary w-full mb-4 rounded-box",
            disabled: uploaded.read().is_empty(),
            onclick: move |_| async move {
                match save().await {
                    Ok(_) => {
                        uploaded.write().clear();
                        state.set(UploadState::Success);
                    }
                    Err(err) => {
                        state.set(UploadState::Error(err.to_string()));
                    }
                }
            },
            "Naloži dokumente"
        }

        match state() {
            UploadState::Idle => rsx! {},
            UploadState::Success => rsx! {
                div {
                    class: "mb-4",
                    AlertSuccess {
                        title: "Dokumenti uspešno naloženi".to_string(),
                        details: "".to_string(),
                    }
                }
            },
            UploadState::Error(error) => rsx! {
                div {
                    class: "mb-4",
                    AlertError {
                        title: "Napaka pri nalaganju dokumentov".to_string(),
                        details: error.to_string(),
                    }
                }
            },
        }

        for file in uploaded.read().iter().cloned().rev() {
            div {
                class: "flex items-center justify-between p-3 mb-2 rounded-box bg-alt-100",
                div {
                    class: "flex items-center",
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
                div {
                    class: "flex items-center",
                    button {
                        class: "cursor-pointer text-base-content/50 hover:text-base-content",
                        onclick: move |_| { remove(&file); },
                        svg {
                            class: "shrink-0 size-4",
                            width: "24",
                            height: "24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            path { d: "M3 6h18" }
                            path { d: "M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" }
                            path { d: "M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" }
                            line { x1: "10", x2: "10", y1: "11", y2: "17"}
                            line {x1: "14", x2: "14", y1: "11", y2: "17"}
                        }
                    }
                }
            }
        }
    }
}
