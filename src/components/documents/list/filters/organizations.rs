use dioxus::prelude::*;

use crate::components::inputs::{InputLabel, InputList, InputSwitch};

#[component]
pub fn FilterOrganizations() -> Element {
    rsx! {
        InputLabel { title: "Organizacije" }
        InputList { id: "organizations" }
        InputSwitch { id: "organizations-condition", label_off: "Unija", label_on: "Presek" }
    }
}
