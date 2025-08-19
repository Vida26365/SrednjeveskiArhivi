#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

use dioxus::desktop::tao::window::Icon;
use dioxus::desktop::wry::http::{Response, StatusCode};
use dioxus::desktop::{WindowBuilder, use_asset_handler};
use dioxus::logger::tracing::{error, info};
use dioxus::prelude::*;
use uuid::Uuid;

use crate::components::navbar::Navbar;
use crate::database::get_database;
use crate::directories::DIRECTORIES;
use crate::views::documents::display::DocumentDisplay;
use crate::views::documents::list::DocumentList;
use crate::views::documents::upload::DocumentUpload;
use crate::views::organizations::list::OrganizationList;
// use crate::views::persons::list::PersonList;

mod components;
mod database;
mod directories;
mod entities;
mod utils;
mod views;

#[derive(Clone, Debug, PartialEq, Eq, Routable)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]

    #[redirect("/", || Route::DocumentList {})]

    #[route("/documents")]
    DocumentList {},

    #[route("/documents/upload")]
    DocumentUpload {},

    #[route("/documents/:id")]
    DocumentDisplay { id: Uuid },

    #[route("/organizations")]
    OrganizationList {},

    // #[route("/persons")]
    // PersonList {},
}

fn main() {
    dioxus::logger::initialize_default();

    let icon = Icon::from_rgba(Vec::from(include_bytes!("../assets/images/icon.rgba")), 128, 128);

    let window = WindowBuilder::new()
        .with_title("Srednjeveški Arhivi")
        .with_window_icon(icon.ok())
        .with_always_on_top(false);

    let config = dioxus::desktop::Config::new()
        .with_resource_directory(DIRECTORIES.sysdata.join("assets"))
        .with_data_directory(DIRECTORIES.userdata.join("webview"))
        .with_window(window);

    let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    runtime.block_on(get_database());

    LaunchBuilder::desktop().with_cfg(config).launch(App)
}

#[component]
fn App() -> Element {
    use_asset_handler("content", |request, responder| {
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
        spawn(async {
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
        document::Link { rel: "icon", href: asset!("/assets/images/icon.ico") }
        document::Link { rel: "icon", href: asset!("/assets/images/icon.svg") }
        document::Link { rel: "stylesheet", href: asset!("/assets/styles/tailwind.css") }

        // Router view
        Router::<Route> {}
    }
}
