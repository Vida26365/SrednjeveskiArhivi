use anyhow::Result;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use sea_orm::EntityTrait;
use uuid::Uuid;

use crate::components::alerts::error::AlertError;
use crate::database::get_database;
use crate::entities::Document;

fn vec_to_multyline(vec: Vec<String>) -> String {
    let mut value = String::new();
    for key in &vec {
        value += &(String::from("\n") + key)
    }
    value
}

#[component]
pub fn DocumentDisplay(id: Uuid) -> Element {
    let document: Resource<Result<_>> = use_resource(move || async move {
        let database = get_database().await;
        Ok(Document::find_by_id(id).one(database).await?)
    });

    match &*document.read_unchecked() {
        Some(Ok(Some(document))) => rsx! {
            document::Link { rel: "stylesheet", href: asset!("/assets/styles/urejanje.css") },
            document::Script { src: asset!("/assets/scripts/grid.js") },
            // script { src: "/assets/scripts/grid.js"}
            div { class: "trije_divi panes pane h-full",
                div { class: "leva_stran pane",
                form { onsubmit: move |event| { info!("Submitted! {event:?}") },
                    ul{
                        li {
                            label { "Ime datoteke:" }
                            label { "{document.filename}" }
                        }
                        li {
                            label { id: "naslov:", "Naslov dokumenta: "}
                            input { id: "naslov", value: document.title.clone()}
                        }
                        li {
                            label {"imena oseb"}
                            ul {
                                padding_left: "30px",
                                list_styler_type: "square",

                                //TODO: Format bo drugačen ko bo implementiran v bazi
                                for name in [Vec::from(["ime1osebe1", "ime2osebe2"]), Vec::from(["oseba2"]), Vec::from(["filip", "še en filip", "pravzaprav so tu kar trije filipi"]), Vec::from(["zdaj se je pa pojavila še ena vida"])] {
                                    li {
                                        list_styler_type: "square",
                                        ul {
                                            for variacije in name {
                                                li {
                                                    input {
                                                        spellcheck: "false",
                                                        value: "{variacije}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        li {
                            label {"Ključne besede: "}
                            textarea { value: vec_to_multyline(document.keywords.0.clone())}
                        }
                        input { r#type: "Submit" }
                    }

                }
                }


                div { class: "srednja_stran pane",
                    form { onsubmit: move |event| { info!("Submitted! {event:?}") },
                        textarea {
                            width: "100%",
                            autocomplete: "false",
                            spellcheck: "false",
                            name: "povzetek",
                            value: document.summary.clone()
                        }
                        textarea {
                            width: "100%",
                            autocomplete: "false",
                            spellcheck: "false",
                            name: "zapis",
                            value: document.content.clone()
                        }
                        input { r#type: "Submit" }
                    }
                }


                div { class: "desna_stran pane",
                    embed {
                        src: "/content/{document.id}#toolbar=0",
                        type: "application/pdf",
                        width: "100%",
                        height: "100%",
                    }

            }

            }
            // div {
            //     h1 { "{document.title}" }
            //     p { b { "Filename: " } "{document.filename}" }
            //     p { b { "Keywords: " } "{document.keywords.0.join(\", \")}" }
            //     p { b { "Summary: " } "{document.summary}" }
            //     embed {
            //         src: "/content/{document.id}#toolbar=0",
            //         type: "application/pdf",
            //         width: "100%",
            //         height: "1000px",
            //     }
            // }
        },
        Some(Ok(None)) => rsx! {
            AlertError {
                title: "Dokument ni najden".to_string(),
                details: "".to_string(),
            }
        },
        Some(Err(error)) => rsx! {
            AlertError {
                title: "Napaka pri nalaganju dokumenta".to_string(),
                details: format!("{:?}", error),
            }
        },
        None => rsx! {
            "Nalaganje dokumenta ..."
        },
    }
}
