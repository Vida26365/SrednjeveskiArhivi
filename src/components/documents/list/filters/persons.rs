use dioxus::prelude::*;

use crate::components::inputs::{InputLabel, InputList, InputSwitch};

#[component]
pub fn FilterPersons() -> Element {
    rsx! {
        InputLabel { title: "Osebe" }
        InputList { id: "persons" }
        InputSwitch { id: "persons-condition", label_off: "Unija", label_on: "Presek" }
    }
}
