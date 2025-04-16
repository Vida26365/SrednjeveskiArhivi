use dioxus::prelude::*;
use sea_orm::EntityTrait;

use crate::database::get_database;
use crate::entities::Document;

#[component]
pub fn Documents() -> Element {
    let documents = use_resource(move || async move {
        let database = get_database().await;
        Document::find().all(database).await
    });

    rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            // You can read resource just like a signal. If the resource is still
            // running, it will return None
            if let Some(response) = &*documents.read() {
                match response {
                    Ok(urls) => rsx! {
                        for image in urls.iter().take(3) {
                                span {
                                    key: "{image.id}",
                                    style: "margin: 10px; padding: 10px; border: 1px solid black;",
                                    "Filename: {image.filename}",
                                }
                        }
                    },
                    Err(err) => rsx! { "Failed to fetch response: {err}" },
                }
            } else {
                "Loading..."
            }
        }
    }


}
