use dioxus::prelude::*;

use crate::components::inputs::{InputLabel, InputList, InputSwitch};

#[component]
pub fn FilterKeywords() -> Element {
    rsx! {
        InputLabel { title: "Ključne besede" }
        InputList { id: "keywords" }
        InputSwitch { id: "keywords-condition", label_off: "Unija", label_on: "Presek" }
    }
}
