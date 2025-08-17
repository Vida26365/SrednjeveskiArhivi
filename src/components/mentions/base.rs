use dioxus::prelude::*;

use crate::Route;
use crate::entities::DocumentModel;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MentionOrder {
    First,
    Last,
}

pub fn render_mention(documents: &[DocumentModel], order: MentionOrder) -> Element {
    let iter = documents.iter().filter_map(|document| document.date.map(|date| (document, date)));

    let selected = match order {
        MentionOrder::First => iter.min_by_key(|&(_, date)| date),
        MentionOrder::Last => iter.max_by_key(|&(_, date)| date),
    };

    match selected {
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
