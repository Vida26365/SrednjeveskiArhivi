use dioxus::prelude::*;

use crate::Route;
use crate::entities::DocumentModel;

#[component]
pub fn MentionFirst(documents: Vec<DocumentModel>) -> Element {
    super::render_mention(&documents, super::MentionOrder::First)
}
