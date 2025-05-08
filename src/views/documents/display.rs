use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use sea_orm::EntityTrait;
use uuid::Uuid;

use crate::components::alerts::error::AlertError;
use crate::database::get_database;
use crate::entities::Document;

// use freyr::prelude::*;

fn vec_to_multyline(vec: Vec<String>) -> String {
    let mut value = String::new();
    for key in &vec {
        value += &(String::from("\n") + key)
    }
    value
}

#[component]
pub fn DocumentDisplay(id: Uuid) -> Element {
    let document = use_resource(move || async move {
        let database = get_database().await;
        Document::find_by_id(id).one(database).await
    });

    // let jeziki = Vec::from([
    //     DropdownItem {label: String::from("Latinščina"), url: None},
    //     DropdownItem {label: String::from("Slovenščina"), url: None},
    //     DropdownItem {label: String::from("Nemščina"), url: None}
    // ]);

    let jeziki = Vec::from([
        "Slovenščina",
        "Latinščina",
        "Nemščina"
    ]);

    // let config_dropdown = DropdownConfig {
    //     title: String::from("Menu"),
    //     label: jeziki,
    //     background_color: DropdownColorScheme::Freyr,
    //     title_color: DropdownTitleColor::Light,
    //     labels_color: DropdownLabelsColor::Dark,
    //     hover_color: DropdownHoverColor::Custom("#03346E"),
    // };

    match &*document.read_unchecked() {
        Some(Ok(Some(document))) => rsx! {
            document::Link { rel: "stylesheet", href: asset!("/assets/styles/urejanje.css") },
            document::Script { src: asset!("/assets/scripts/grid.js") },
            // script { src: "/assets/scripts/grid.js"}
            div { class: "trije_divi panes pane h-full",
                div { class: "leva_stran pane",
                form { onsubmit: move |event| { info!("Submitted! {event:?}") },
                    // TODO: povsod v input treba dodati value oziroma ime
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
                            label {"Datum"} //TODO: Kakšen format inma datum
                            // input { value: to_string(document.date.clone()) }
                        }
                        li {
                            label {"imena oseb"}
                            ul {
                                padding_left: "30px",
                                list_styler_type: "square",

                                //TODO: Format bo drugačen ko bo implementiran v bazi
                                for name in [Vec::from(["ime1osebe1", "ime2osebe2"]), Vec::from(["oseba2"]), Vec::from(["filip", "še en filip", "pravzaprav so tu kar trije filipi"])] {
                                    li {
                                        list_styler_type: "square",
                                        ul {
                                            for variacije in name {
                                                //TODO: variacije v svojem text area
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
                            label { "Lokacija: " }
                            //TODO: Glavna lokacija i ostale lokacije
                        }
                        li {
                            label {"Ključne besede: "}
                            textarea { value: vec_to_multyline(document.keywords.0.clone())}
                        }
                        li {
                            label {"Jeziki"}
                            select {
                                for jezik in &jeziki {
                                    option {
                                        value: *jezik,
                                        "{jezik}"
                                    }
                                }
                            }
                            //TODO: gumb dodaj jezik

                        }
                        li { input { r#type: "Submit", "shrani" } }
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
