use dioxus::events::Key::Enter;
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use dioxus_heroicons::IconShape;

use crate::entities::{DocumentModel, PersonAliasModel, PersonModel};
type DocumentParam = Signal<DocumentModel>;
type PersonsParam = Signal<Vec<(PersonModel, Vec<PersonAliasModel>)>>;

#[component]
pub fn InputPersons(document: DocumentParam, persons: PersonsParam) -> Element {
    // let mut persons = use_signal(move || {
    //     document.read().persons.0.clone()
    //     // persons.read().clone().into_iter().map(|(person, _)| person.name).collect::<Vec<_>>()
    // });
    let persons = use_signal(move || {
        persons
            .read()
            .clone()
            .into_iter()
            .map(|(person, alliases)| {
                (person.name, alliases.iter().map(|alias| alias.name.clone()).collect::<Vec<_>>())
            })
            .collect::<Vec<_>>()
    });

    // let mut alliases: Signal<Vec<Vec<String>>> = use_signal(Vec::new);

    rsx! {
        label {
            class: "flex pb-2 font-semibold",
            "Osebe"
        }
        SubListInput { string_vec_list: persons }
        VaskiPosebnez { persons: persons }
    }
}

#[component]
fn SubListInput(string_vec_list: Signal<Vec<(String, Vec<String>)>>) -> Element {
    rsx!(for (index, (glavno_ime, variacije)) in string_vec_list.read().iter().cloned().enumerate()
    {
        VmesnaKomponentaKerjeRustKrneki {
            variacije,
            glavno_ime: glavno_ime.clone(),
            string_vec_list: string_vec_list.clone(),
            index,
        }
    })
}

#[component]
fn VmesnaKomponentaKerjeRustKrneki(
    variacije: Vec<String>,
    glavno_ime: String,
    string_vec_list: Signal<Vec<(String, Vec<String>)>>,
    index: usize,
) -> Element {
    let mut variacije = use_signal(|| variacije);
    rsx!(
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
                        oninput: move |event| {
                            let mut string_vec_list = string_vec_list.write();
                            string_vec_list[index].0 = event.value();
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
                        onclick: move |_event| {
                            variacije.write().push(String::from(""));
                        },
                        "+"

                    }
                }
            }
            div {
                margin_left: "15%",
                Kaaj {
                    variacije: variacije,
                    glavno_ime: glavno_ime.clone()
                }

            }
        }
    )
}

#[component]
fn Kaaj(variacije: Signal<Vec<String>>, glavno_ime: String) -> Element {
    rsx!(
        for (index, vzdevek) in variacije.read().iter().cloned().enumerate() {
            div {
                class: "input w-max-75 mb-2",
                input {
                    aria_autocomplete: "none",
                    autocapitalize: "false",
                    autocomplete: "false",
                    spellcheck: "false",
                    name: "persons/{glavno_ime}",
                    value: "{vzdevek}",

                    oninput: move |event| {
                        let mut variacije = variacije.write();
                        variacije[index] = event.value();
                    },

                    onkeypress: move |event| {
                        if event.key() == Enter {
                            event.prevent_default();
                        }
                    }
                }
                button {
                    class: "cursor-pointer text-base-content/50 hover:text-base-content",
                    onclick: move |event| {
                        event.prevent_default();
                        variacije.write().remove(index);
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
        }
    )
}

#[component]
fn VaskiPosebnez(persons: Signal<Vec<(String, Vec<String>)>>) -> Element {
    let mut additional = use_signal(String::new);
    let mut dodatne_variacije = use_signal(Vec::new);

    rsx!(
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
                        value: "{additional}",
                        oninput: move |event| {
                            additional.set(event.value());
                        },
                        onkeypress: move |event| {
                            if event.key() == Enter {
                                event.prevent_default();
                                persons.write().push((additional.read().clone(), dodatne_variacije.read().clone()));
                                additional.set(String::new());
                                dodatne_variacije.set(Vec::new());
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
                        onclick: move |_event| {
                                dodatne_variacije.write().push(String::from(""));
                            },
                            "+"
                    }
                }
            }
            Kaaj {
                variacije: dodatne_variacije,
                glavno_ime: additional.read()
            }
        }
    )
}
