use dioxus::prelude::*;

use crate::components::inputs::{InputLabel, InputList, InputSwitch};

#[component]
pub fn FilterLocations() -> Element {
    rsx! {
        InputLabel { title: "Lokacije" }
        InputList { id: "locations" }
        InputSwitch { id: "locations-condition", label_off: "Unija", label_on: "Presek" }
    }
}
