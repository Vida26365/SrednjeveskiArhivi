use std::println;

use dioxus::events::Key::Enter;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use dioxus_heroicons::IconShape;

use crate::entities::{document::Persons, DocumentModel, PersonAliasModel, PersonModel};
type DocumentParam = Signal<DocumentModel>;
type PersonsParam = Signal<Vec<(PersonModel, Vec<PersonAliasModel>)>>;

#[component]
pub fn InputPersons(document: DocumentParam, persons: PersonsParam) -> Element {
    // let mut persons = use_signal(move || {
    //     document.read().persons.0.clone()
    //     // persons.read().clone().into_iter().map(|(person, _)| person.name).collect::<Vec<_>>()
    // });
    let mut persons = use_signal(move || {
        persons
            .read()
            .clone()
            .into_iter()
            .map(|(person, alliases)| {
                (person.name, use_signal(|| alliases.iter().map(|alias| alias.name.clone()).collect::<Vec<_>>()))
            })
            .collect::<Vec<_>>()
    });
    let mut additional = use_signal(String::new);

    // let mut alliases: Signal<Vec<Vec<String>>> = use_signal(Vec::new);

    rsx! {
        label {
            class: "flex pb-2 font-semibold",
            "Osebe"
        }
        // TUKI je blo prej
        SubListInput {
            string_vec_list: persons
        }
        div {
            class: "flex gap-2",
            div {
                class: "input w-full",
                input {
                    aria_autocomplete: "none",
                    autocapitalize: "false",
                    autocomplete: "false",
                    spellcheck: "false",
                    name: "persons",
                    value: "{additional}",
                    oninput: move |event| {
                        additional.set(event.value());
                    },
                    onkeypress: move |event| {
                        if event.key() == Enter {
                            event.prevent_default();
                            persons.write().push((additional.read().clone(), use_signal(Vec::new)));
                            additional.set(String::new());
                        }
                    }
                }
                button {
                    class: "cursor-pointer text-base-content/50 hover:text-base-content",
                    onclick: move |event| {
                        event.prevent_default();
                        additional.set(String::new());
                    },
                    svg {
                        class: "size-4 shrink-0",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        { Shape::Trash.path() }
                    }
                }
            }
            div {
                button {
                    class: "cursor-pointer text-base-content/50 hover:text-base-content",
                    onclick: move |event| info!("Gumb {event:?}"), "+"
                }
            }
        }
    }
}


#[component]
fn SubListInput(string_vec_list: Signal<Vec<(String, Signal<Vec<String>>)>>) -> Element {

    rsx!(
        for (glavno_ime, variacije) in string_vec_list.read().iter().cloned() {
            div {
                div {
                    class: "flex gap-2",
                    div {
                        class: "input w-full mb-2",
                        input {
                            aria_autocomplete: "none",
                            autocapitalize: "false",
                            autocomplete: "false",
                            spellcheck: "false",
                            name: "persons",
                            value: "{glavno_ime}",
                            oninput: {
                                let oseba = glavno_ime.clone();
                                move |event: Event<FormData>| {
                                    let mut persons = string_vec_list.write();
                                    match persons.clone().into_iter().map(|(oseba, _)| oseba).position(|existing| existing == oseba) {
                                        Some(pos) => persons[pos] = (event.value(), variacije),
                                        None => persons.push((event.value(), use_signal(Vec::new))),
                                    }
                                }
                            },
                            onkeypress: move |event| {
                                if event.key() == Enter {
                                    event.prevent_default();
                                }
                            }
                        }

                        button {
                            class: "cursor-pointer text-base-content/50 hover:text-base-content",
                            onclick: {
                                let oseba = glavno_ime.clone();
                                move |event: Event<MouseData>| {
                                    event.prevent_default();
                                    string_vec_list.write().retain(|existing| existing.0 != oseba);
                                }
                            },
                            svg {
                                class: "size-4 shrink-0",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                { Shape::Trash.path() }
                            }
                        }
                    }
                    div {
                        button {
                            class: "cursor-pointer text-base-content/50 hover:text-base-content",
                            onclick: move |event| {
                                event.prevent_default();
                                string_vec_list.write().iter_mut().for_each(|(person, aliases)| {
                                    if person == &glavno_ime {
                                        aliases.push(String::from(""));
                                    }
                                });
                            },
                            "+"

                        }
                    }
                }
                div {
                    margin_left: "15%",
                    Kaaj {
                        string_vec_list: string_vec_list.clone(),
                        glavno_ime: glavno_ime.clone()
                    }

                }
            }
        }
    )
}


#[component]
fn Kaaj(string_vec_list: Signal<Vec<(String, Signal<Vec<String>>)>>, glavno_ime: String) -> Element {

    let mut variacije = match string_vec_list
        .read()
        .iter()
        .find(|(main_name, _)| main_name == &glavno_ime)
        {
            Some((_, variations)) => *variations,
            None => use_signal(Vec::new),
        };

    let mut additional_variations: Signal<String> = use_signal(String::new);

    rsx!(
        for vzdevek in variacije.read().iter().cloned() {
            div {
                class: "input w-max-75 mb-2",
                input {
                    aria_autocomplete: "none",
                    autocapitalize: "false",
                    autocomplete: "false",
                    spellcheck: "false",
                    name: "{glavno_ime}",
                    value: "{vzdevek}",

                    oninput: {
                        let variacija = glavno_ime.clone();
                        move |event: Event<FormData>| {
                            let mut persons = variacije.write();
                            match persons.clone().into_iter().map(|oseba| oseba).position(|existing| existing == variacija) {
                                Some(pos) => persons[pos] = event.value(),
                                None => persons.push(event.value())
                            }
                        }
                    },

                    onkeypress: move |event| {
                        if event.key() == Enter {
                            event.prevent_default();
                        }
                    }
                }
            }
        }
        div {
            class: "flex gap-2",
            div {
                class: "input w-full",
                input {
                    aria_autocomplete: "none",
                    autocapitalize: "false",
                    autocomplete: "false",
                    spellcheck: "false",
                    name: "persons",
                    value: "{additional_variations}",
                    oninput: move |event| {
                        additional_variations.set(event.value());
                    },
                    onkeypress: move |event| {
                        if event.key() == Enter {
                            event.prevent_default();
                            variacije.write().push(additional_variations.read().clone());
                            additional_variations.set(String::new());
                        }
                    }
                }
                button {
                    class: "cursor-pointer text-base-content/50 hover:text-base-content",
                    onclick: move |event| {
                        event.prevent_default();
                        additional_variations.set(String::new());
                    },
                    svg {
                        class: "size-4 shrink-0",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        { Shape::Trash.path() }
                    }
                }
            }
            div {
                button {
                    class: "cursor-pointer text-base-content/50 hover:text-base-content",
                    onclick: move |event| info!("Gumb {event:?}"), "+"
                }
            }
        }
    )
}

