use dioxus::prelude::*;

use crate::Route;
use crate::entities::DocumentModel;

#[component]
pub fn MentionFirst(documents: Vec<DocumentModel>) -> Element {
    let first = documents
        .iter()
        .filter_map(|document| document.date.map(|date| (document, date)))
        .min_by_key(|&(_, date)| date);

    match first {
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
