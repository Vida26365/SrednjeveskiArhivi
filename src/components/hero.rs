use std::path::PathBuf;

use dioxus::prelude::*;

const HEADER: Asset = asset!("/assets/images/header.svg");

#[component]
pub fn Hero() -> Element {
    let path = String::from("C:\\Users\\fs90700\\Documents\\SrednjeveskiArhivi\\zapisi\\GZL I-1 (1243 april 13)-1.jpg");

    rsx! {
        div {
            id: "hero",
            img { src: HEADER, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}
