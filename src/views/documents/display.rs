use dioxus::prelude::*;
use dioxus::desktop::{use_asset_handler, wry::http::Response};

#[component]
pub fn DocumentDisplay(id: uuid::Uuid) -> Element {
    use_asset_handler("documents", |request, response| {
        // We get the original path - make sure you handle that!
        if request.uri().path() != "/logos/logo.png" {
            return;
        }

        response.respond(Response::new(include_bytes!("./assets/logo.png").to_vec()));
    });

    rsx! {
        div {
            id: "document-display",
            h1 { "Document Display" }
            p { "This is the document display page." }
            // Add more content here as needed
        }
    }
}
