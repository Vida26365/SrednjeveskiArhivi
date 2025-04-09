use dioxus::prelude::*;

use crate::components::Navbar;
use crate::database::get_database;
use crate::directories::DIRECTORIES;
use crate::views::{Blog, Home};

mod components;
mod database;
mod directories;
mod entities;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/images/favicon.ico");
const TAILWIND: Asset = asset!("/assets/styles/tailwind.css");

fn main() {
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    runtime.block_on(get_database());

    let config = dioxus::desktop::Config::new()
        .with_resource_directory(DIRECTORIES.sysdata.join("assets"))
        .with_data_directory(DIRECTORIES.userdata.join("webview"));

    dioxus::LaunchBuilder::desktop().with_cfg(config).launch(App)
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND }

        // Router view
        Router::<Route> {}
    }
}
