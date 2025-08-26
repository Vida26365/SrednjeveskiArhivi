use std::sync::Arc;

use dioxus::html::{FileEngine, HasFileData};
use dioxus::prelude::*;
use dioxus_heroicons::IconShape;
use dioxus_heroicons::outline::Shape;

#[component]
pub fn FileUpload(
    handler: Callback<Arc<dyn FileEngine>>,
    #[props(default = None)] accept: Option<String>,
    #[props(default = false)] multiple: bool,
    #[props(default = false)] directory: bool,
) -> Element {
    let mut hovered = use_signal(|| false);
    let background = use_memo(move || if hovered() { "bg-alt-300" } else { "bg-alt-100" });

    rsx! {
        label {
            class: "flex flex-col items-center justify-center h-64 w-full rounded-box cursor-pointer {background} hover:bg-alt-200",
            for: "upload",
            ondragover: move |event| {
                event.prevent_default();
                hovered.set(true);
            },
            ondragleave: move |event| {
                event.prevent_default();
                hovered.set(false);
            },
            ondrop: move |event| {
                event.prevent_default();
                hovered.set(false);
                if let Some(files) = event.files() { handler.call(files) }
            },
            svg {
                class: "size-12 mb-4",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                view_box: "0 0 24 24",
                { Shape::ArrowUpTray.path() }
            }
            p {
                class: "mb-2 text-sm",
                span { class: "font-semibold", "Kliknite za nalaganje" }
                " ali povlecite in spustite"
            }
            input {
                type: "file",
                id: "upload",
                class: "hidden",
                accept: if let Some(accept) = accept { accept },
                multiple: multiple,
                directory: directory,
                onchange: move |event| {
                    if let Some(files) = event.files() { handler.call(files) }
                },
            }
        }
    }
}
