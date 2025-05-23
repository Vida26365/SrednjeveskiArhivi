use dioxus::prelude::*;

use crate::entities::DocumentModel;

#[component]
pub fn PanePdf(document: Signal<DocumentModel>) -> Element {
    rsx! {
        embed {
            src: "/content/{document.read().id}#toolbar=0",
            type: "application/pdf",
            width: "100%",
            height: "100%",
        }
    }
}
