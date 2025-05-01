use std::fs;

use dioxus::desktop::use_asset_handler;
use dioxus::desktop::wry::http::{Response, StatusCode};
use dioxus::logger::tracing::{error, info};
use dioxus::prelude::*;
use uuid::Uuid;
use dioxus::desktop::WindowBuilder;
// use dioxus::desktop::PhysicalPosition;

use crate::components::navbar::Navbar;
use crate::database::get_database;
use crate::directories::DIRECTORIES;
use crate::views::blog::Blog;
use crate::views::documents::display::DocumentDisplay;
use crate::views::documents::list::DocumentList;

mod components;
mod database;
mod directories;
mod entities;
mod pdf_to_text;
mod views;

#[derive(Clone, Debug, PartialEq, Eq, Routable)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]

    #[redirect("/", || Route::DocumentList {})]

    #[route("/documents")]
    DocumentList {},

    #[route("/documents/:id")]
    DocumentDisplay { id: uuid::Uuid },

    #[route("/blog/:id")]
    Blog { id: i32 },
}

const ICON: Asset = asset!("/assets/images/icon.ico");
const TAILWIND: Asset = asset!("/assets/styles/tailwind.css");
const UREJANJE: Asset = asset!("/assets/styles/urejanje.css");

fn main() {
    dioxus::logger::initialize_default();

    let config = dioxus::desktop::Config::new()
        .with_resource_directory(DIRECTORIES.sysdata.join("assets"))
        .with_data_directory(DIRECTORIES.userdata.join("webview"))
        .with_window(make_window());

    let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    runtime.block_on(get_database());

    LaunchBuilder::desktop().with_cfg(config).launch(App)
}

#[component]
fn App() -> Element {
    use_asset_handler("content", move |request, responder| {
        // Validate the request URL
        let Ok(id) = Uuid::parse_str(request.uri().path().trim_start_matches("/content/")) else {
            let response =
                Response::builder().status(StatusCode::BAD_REQUEST).body(vec![]).unwrap();
            responder.respond(response);
            return;
        };

        // Construct the document path
        let storage = DIRECTORIES.userdata.join("documents");
        let path = storage.join(id.to_string()).with_extension("pdf");

        info!("Loading document from path: {:?}", path);

        // Load the document content
        spawn(async move {
            match fs::read(path) {
                Ok(content) => {
                    let response = Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", "application/pdf")
                        .body(content)
                        .unwrap();
                    responder.respond(response);
                }
                Err(error) => {
                    error!("Failed to load document: {:?}", error);

                    let response =
                        Response::builder().status(StatusCode::NOT_FOUND).body(vec![]).unwrap();
                    responder.respond(response);
                }
            }
        });
    });

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: ICON }
        document::Link { rel: "stylesheet", href: TAILWIND }
        document::Link { rel: "stylesheet", href: UREJANJE }

        // Router view
        Router::<Route> {}
    }
}


fn make_window() -> WindowBuilder {
    WindowBuilder::new()
        // .with_transparent(true)
        // .with_decorations(false)
        // .with_resizable(false)
        .with_always_on_top(false)
        // .with_position(PhysicalPosition::new(0, 0))
        // .with_max_inner_size(LogicalSize::new(100000, 50))
}
