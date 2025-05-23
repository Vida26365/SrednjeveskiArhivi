use dioxus::prelude::*;

use crate::entities::DocumentModel;
use crate::Route;

#[component]
pub fn MentionLast(documents: Vec<DocumentModel>) -> Element {
    let last = documents
        .iter()
        .filter_map(|document| document.date.map(|date| (document, date)))
        .max_by_key(|&(_, date)| date);

    match last {
        Some((document, date)) => rsx! {
            Link {
                to: Route::DocumentDisplay { id: document.id },
                "{date.year()}"
            }
        },
        None => rsx! {
            span { "/" }
        },
    }
}
