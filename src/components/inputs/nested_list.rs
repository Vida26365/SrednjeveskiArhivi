use dioxus::events::Key::Enter;
use dioxus::prelude::*;
use dioxus_heroicons::IconShape;
use dioxus_heroicons::outline::Shape;

#[component]
pub fn InputNestedList(
    id: &'static str,
    #[props(into, default = vec![])] values: Vec<(Option<String>, Vec<String>)>,
    // test_values: Vec<(Option<String>, Vec<String>)>,
) -> Element {
    let values = use_signal(|| values);
    // let test_values = use_signal(|| test_values);

    rsx! {
        fieldset {
            class: "space-y-2",
            id: "{id}",

            for (index, (name, aliases)) in values.read().iter().cloned().enumerate() {
                if name.is_none() {
                    EmptyComponent {}
                }
                else {
                    ListItemOziromaVidaNeZnaRusta { id, index, name: name.unwrap(), aliases, values }
                }
            }

            // for (index, (name, aliases)) in values.read().iter().cloned().enumerate() {
            //     if name == "StringKiSeNikoliNeBoNapisalVAmplikacijii" {
            //         EmptyComponent {}
            //     }
            //     else {
            //         ListItemOziromaVidaNeZnaRusta { id, index, name, aliases, values }
            //     }
            // }

            LastItemOziromaVaskiPosebnez { id, values }
        }
    }
}

#[component]
fn EmptyComponent() -> Element {
    rsx! {}
}

#[component]
fn ListItemOziromaVidaNeZnaRusta(
    id: &'static str,
    index: usize,
    name: ReadOnlySignal<String>,
    aliases: Vec<String>,
    values: Signal<Vec<(Option<String>, Vec<String>)>>,
) -> Element {
    let mut aliases = use_signal(|| aliases);

    rsx! {
        div {
            class: "flex gap-2",

            div {
                class: "input w-full",
                input {
                    aria_autocomplete: "none",
                    autocapitalize: "false",
                    autocomplete: "false",
                    spellcheck: "false",
                    name: "{id}",
                    value: "{name}",
                    oninput: move |event| {
                        let mut values = values.write();
                        values[index].0 = Some(event.value());
                    },
                    onkeypress: move |event| {
                        if event.key() == Enter {
                            event.prevent_default();
                        }
                    }
                }
                button {
                    class: "cursor-pointer text-base-content/50 hover:text-base-content",
                    type: "button",
                    onclick: move |_| {
                        // values.write().remove(index);
                        values.write()[index] = (None, vec![]);
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
                    class: "btn btn-square btn-soft",
                    type: "button",
                    onclick: move |_| {
                        aliases.write().push(String::new());
                    },
                    "+"
                }
            }
        }

        div {
            class: "ml-8 space-y-2",
            SubInputListOziromaKaaj { id, name, aliases }
        }
    }
}

#[component]
fn LastItemOziromaVaskiPosebnez(
    id: &'static str,
    values: Signal<Vec<(Option<String>, Vec<String>)>>,
) -> Element {
    let mut name = use_signal(String::new);
    let mut aliases = use_signal(Vec::new);

    rsx! {
        div {
            class: "flex gap-2",

            div {
                class: "input w-full",
                input {
                    aria_autocomplete: "none",
                    autocapitalize: "false",
                    autocomplete: "false",
                    spellcheck: "false",
                    name: "{id}",
                    value: "{name}",
                    oninput: move |event| {
                        name.set(event.value());
                    },
                    onkeypress: move |event| {
                        if event.key() == Enter {
                            event.prevent_default();
                            values.write().push((Some(name.read().clone()), aliases.read().clone()));
                            name.set(String::new());
                            aliases.set(Vec::new());
                        }
                    }
                }
                button {
                    class: "cursor-pointer text-base-content/50 hover:text-base-content",
                    type: "button",
                    onclick: move |_| {
                        name.set(String::new());
                        aliases.set(Vec::new());
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
                    class: "btn btn-square btn-soft",
                    type: "button",
                    onclick: move |_| {
                        aliases.write().push(String::new());
                    },
                    "+"
                }
            }
        }

        div {
            class: "ml-8 space-y-2",
            SubInputListOziromaKaaj { id, name, aliases }
        }
    }
}

#[component]
fn SubInputListOziromaKaaj(
    id: &'static str,
    name: ReadOnlySignal<String>,
    aliases: Signal<Vec<String>>,
) -> Element {
    rsx! {
        for (index, alias) in aliases.read().iter().cloned().enumerate() {
            div {
                class: "input w-full",
                input {
                    aria_autocomplete: "none",
                    autocapitalize: "false",
                    autocomplete: "false",
                    spellcheck: "false",
                    name: "{id}/{name}",
                    value: "{alias}",
                    oninput: move |event| {
                        let mut aliases = aliases.write();
                        aliases[index] = event.value();
                    },
                    onkeypress: move |event| {
                        if event.key() == Enter {
                            event.prevent_default();
                        }
                    }
                }
                button {
                    class: "cursor-pointer text-base-content/50 hover:text-base-content",
                    type: "button",
                    onclick: move |_| {
                        aliases.write().remove(index);
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
