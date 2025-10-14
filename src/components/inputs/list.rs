use dioxus::events::Key::Enter;
use dioxus::prelude::*;
use dioxus_heroicons::IconShape;
use dioxus_heroicons::outline::Shape;

/// InputList is a Dioxus component for managing a dynamic list of text inputs.
/// - `id`: Unique identifier for the input group.
/// - `values`: Initial values for the list (defaults to empty).
#[component]
pub fn InputList(
    id: &'static str,
    #[props(into, default = vec![])] values: Vec<String>,
) -> Element {
    // Signal holding the current list of values
    let mut values = use_signal(|| values);
    // Signal for the value of the "add new" input field
    let mut additional = use_signal(String::new);

    rsx! {
        // The fieldset groups all inputs together visually and semantically
        fieldset {
            class: "space-y-2",
            id: "{id}",

            // Loop through each value in the list and render an input and remove button
            for (index, value) in values.read().iter().enumerate() {
                div {
                    class: "input w-full",
                    input {
                        // Editable input for each value in the list
                        aria_autocomplete: "none",
                        autocapitalize: "false",
                        autocomplete: "false",
                        spellcheck: "false",
                        name: "{id}",
                        value: "{value}",
                        // Update the value in the list when the input changes
                        oninput: move |event| {
                            let mut values = values.write();
                            values[index] = event.value();
                        },
                        // Prevent form submission when pressing Enter
                        onkeypress: move |event| {
                            if event.key() == Enter {
                                event.prevent_default();
                            }
                        }
                    }
                    button {
                        // Button to remove this value from the list
                        class: "cursor-pointer text-base-content/50 hover:text-base-content",
                        type: "button",
                        onclick: move |_| {
                            values.write().remove(index);
                        },
                        // Trash icon for remove button
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

            // Section for adding a new value to the list
            div {
                class: "input w-full",
                input {
                    // Input for typing a new value to add
                    aria_autocomplete: "none",
                    autocapitalize: "false",
                    autocomplete: "false",
                    spellcheck: "false",
                    name: "{id}",
                    value: "{additional}",
                    // Update the "additional" signal as the user types
                    oninput: move |event| {
                        additional.set(event.value());
                    },
                    // On Enter, add the new value to the list and clear the input
                    onkeypress: move |event| {
                        if event.key() == Enter {
                            event.prevent_default();
                            values.write().push(additional.read().clone());
                            additional.set(String::new());
                        }
                    }
                }
                button {
                    // Button to clear the "additional" input field
                    class: "cursor-pointer text-base-content/50 hover:text-base-content",
                    type: "button",
                    onclick: move |_| {
                        additional.set(String::new());
                    },
                    // Trash icon for clear button
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
