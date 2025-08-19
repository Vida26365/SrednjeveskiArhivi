use dioxus::events::Key::Enter;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use dioxus_heroicons::IconShape;
use dioxus_heroicons::outline::Shape;

use crate::components::documents::display::{DocumentSignal, PersonsSignal};

#[component]
pub fn InputPersons(document: DocumentSignal, persons: PersonsSignal) -> Element {
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
                (person.name, alliases.iter().map(|alias| alias.name.clone()).collect::<Vec<_>>())
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
        for (oseba, aliases) in persons.read().iter().cloned() {
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
                            value: "{oseba}",
                            oninput: {
                                let oseba = oseba.clone();
                                move |event: Event<FormData>| {
                                    let mut persons = persons.write();
                                    match persons.clone().into_iter().map(|(oseba, _)| oseba).position(|existing| existing == oseba) {
                                        Some(pos) => persons[pos] = (event.value(), aliases.clone()), //EEEEEEmmm kaj?
                                        None => persons.push((event.value(), Vec::new())),
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
                                let oseba = oseba.clone();
                                move |event: Event<MouseData>| {
                                    event.prevent_default();
                                    persons.write().retain(|existing| existing.0 != oseba);
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
                            onclick:
                                // let oseba_clone = oseba.clone();
                                move |event| {
                                    event.prevent_default();
                                    // persons.write().push((additional.read().clone(), Vec::new()));
                                    // aliases.push(String::from("value"));
                                    persons.write().iter_mut().for_each(|(person, aliases)| {
                                        if *person == oseba {
                                        aliases.push(String::new());
                                    }
                                });
                                println!("Aliases: {aliases:?}");
                                println!("Persons: {persons:?}");
                            },
                            "+"
                        }
                    }
                }
                div {
                    margin_left: "15%",
                    for vzdevek in aliases.iter().cloned() {
                        div {
                            class: "input w-max-75 mb-2",
                            input {
                                aria_autocomplete: "none",
                                autocapitalize: "false",
                                autocomplete: "false",
                                spellcheck: "false",
                                name: "{oseba}",
                                value: "{vzdevek}",
                                oninput: {
                                let vzdevek = vzdevek.clone();
                                let oseba_clone = oseba.clone();
                                move |event: Event<FormData>| {
                                    let mut persons = persons.write();
                                    match persons.clone().into_iter().position(|personvec| personvec.0 == oseba_clone) {
                                        Some(pos) => {
                                            match persons[pos].1.iter().position(|alias| alias == &vzdevek) {
                                                Some(alias_pos) => persons[pos].1[alias_pos] = event.value(), // Update existing alias
                                                None => persons[pos].1.push(event.value()), // Add new alias
                                            }
                                        }
                                        // persons[pos] = (event.value(), Vec::new()), //EEEEEEmmm kaj?
                                        None => persons.push((event.value(), Vec::new())),
                                    }
                                }
                            },
                            }
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
                    value: "{additional}",
                    oninput: move |event| {
                        additional.set(event.value());
                    },
                    onkeypress: move |event| {
                        if event.key() == Enter {
                            event.prevent_default();
                            persons.write().push((additional.read().clone(), Vec::new()));
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
