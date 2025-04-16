use dioxus::prelude::*;

use crate::components::Documents;

#[component]
pub fn Home() -> Element {
    rsx! {
        Documents {}
    }
}
