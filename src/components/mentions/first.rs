use dioxus::prelude::*;

use crate::entities::document;
use crate::Route;

#[component]
pub fn MentionFirst(documents: Vec<document::Model>) -> Element {
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
