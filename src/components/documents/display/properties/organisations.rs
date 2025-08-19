use dioxus::events::Key::Enter;
use dioxus::prelude::*;
use dioxus_heroicons::IconShape;
use dioxus_heroicons::outline::Shape;

use crate::components::documents::display::OrganizationsSignal;

#[component]
pub fn InputOrganisations(organizations: OrganizationsSignal) -> Element {
    let mut organisations = use_signal(move || {
        organizations
            .read()
            .clone()
            .into_iter()
            .map(|(organization, _)| organization.name)
            .collect::<Vec<_>>()
    });
    let mut additional = use_signal(String::new);

    rsx! {
        label {
            class: "flex pb-2 font-semibold",
            "Organizacije"
        }

        for organizacija in organisations.read().iter().cloned() {
            div {
                class: "input w-full mb-2",
                input {
                    aria_autocomplete: "none",
                    autocapitalize: "false",
                    autocomplete: "false",
                    spellcheck: "false",
                    name: "organisations",
                    value: "{organizacija}",
                    oninput: {
                        let organisation = organizacija.clone();
                        move |event: Event<FormData>| {
                            let mut organisations = organisations.write();
                            match organisations.iter().position(|existing| existing == &organisation) {
                                Some(pos) => organisations[pos] = event.value(),
                                None => organisations.push(event.value()),
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
                        let organisation = organizacija.clone();
                        move |event: Event<MouseData>| {
                            event.prevent_default();
                            organisations.write().retain(|existing| existing != &organisation);
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
        }

        div {
            class: "input w-full",
            input {
                aria_autocomplete: "none",
                autocapitalize: "false",
                autocomplete: "false",
                spellcheck: "false",
                name: "organisations",
                value: "{additional}",
                oninput: move |event| {
                    additional.set(event.value());
                },
                onkeypress: move |event| {
                    if event.key() == Enter {
                        event.prevent_default();
                        organisations.write().push(additional.read().clone());
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
    }
}
