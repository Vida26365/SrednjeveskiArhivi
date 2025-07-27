use dioxus::prelude::*;

use crate::components::documents::display::DocumentSignal;

#[component]
pub fn PanePdf(#[props(into)] document: DocumentSignal) -> Element {
    rsx! {
        embed {
            src: "/content/{document.read().id}#toolbar=0&statusbar=0&view=FitH",
            type: "application/pdf",
            width: "100%",
            height: "100%",
        }
    }
}
