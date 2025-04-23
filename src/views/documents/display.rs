use dioxus::prelude::*;
use sea_orm::EntityTrait;
use uuid::Uuid;

use crate::components::alerts::error::AlertError;
use crate::database::get_database;
use crate::entities::Document;

#[component]
pub fn DocumentDisplay(id: Uuid) -> Element {
    let document = use_resource(move || async move {
        let database = get_database().await;
        Document::find_by_id(id).one(database).await
    });

    match &*document.read_unchecked() {
        Some(Ok(Some(document))) => rsx! {
            div {
                h1 { "{document.title}" }
                p { b { "Filename: " } "{document.filename}" }
                p { b { "Keywords: " } "{document.keywords.0.join(\", \")}" }
                if let Some(summary) = &document.summary {
                    p { b { "Summary: " } "{summary}" }
                }
                embed {
                    src: "/content/{document.id}#toolbar=0",
                    type: "application/pdf",
                    width: "100%",
                    height: "1000px",
                }
            }
        },
        Some(Ok(None)) => rsx! {
            AlertError {
                title: "Dokument ni najden".to_string(),
                details: "".to_string(),
            }
        },
        Some(Err(error)) => rsx! {
            AlertError {
                title: "Napaka pri nalaganju dokumenta".to_string(),
                details: error.to_string(),
            }
        },
        None => rsx! {
            "Nalaganje dokumenta ..."
        },
    }
}
