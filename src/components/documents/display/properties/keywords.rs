use dioxus::events::Key::Enter;
use dioxus::prelude::*;
use dioxus_heroicons::IconShape;
use dioxus_heroicons::outline::Shape;

use crate::components::documents::display::DocumentSignal;

#[component]
pub fn InputKeywords(document: DocumentSignal) -> Element {
    let mut keywords = use_signal(move || document.read().keywords.0.clone());
    let mut additional = use_signal(String::new);

    rsx! {
        label {
            class: "flex pb-2 font-semibold",
            "Ključne besede"
        }

        fieldset {
            class: "space-y-2",

            for keyword in keywords.read().iter().cloned() {
                div {
                    class: "input w-full",
                    input {
                        aria_autocomplete: "none",
                        autocapitalize: "false",
                        autocomplete: "false",
                        spellcheck: "false",
                        name: "keywords",
                        value: "{keyword}",
                        oninput: {
                            let keyword = keyword.clone();
                            move |event: Event<FormData>| {
                                let mut keywords = keywords.write();
                                match keywords.iter().position(|existing| existing == &keyword) {
                                    Some(pos) => keywords[pos] = event.value(),
                                    None => keywords.push(event.value()),
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
                            let keyword = keyword.clone();
                            move |event: Event<MouseData>| {
                                event.prevent_default();
                                keywords.write().retain(|existing| existing != &keyword);
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
                    name: "keywords",
                    value: "{additional}",
                    oninput: move |event| {
                        additional.set(event.value());
                    },
                    onkeypress: move |event| {
                        if event.key() == Enter {
                            event.prevent_default();
                            keywords.write().push(additional.read().clone());
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
}
