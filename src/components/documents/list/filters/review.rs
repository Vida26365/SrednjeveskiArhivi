use dioxus::prelude::*;
use sea_orm::Iterable;

use crate::components::inputs::InputLabel;
use crate::entities::document::ReviewStatus;

#[component]
pub fn FilterReview() -> Element {
    rsx! {
        InputLabel { title: "Stanje" }
        fieldset {
            class: "space-y-2",
            for review in ReviewStatus::iter() {
                div {
                    class: "whitespace-nowrap",
                    input {
                        class: "checkbox",
                        type: "checkbox",
                        name: "review",
                        id: "review-{review.as_variant_name()}",
                        value: "{review.as_variant_name()}",
                    }
                    label {
                        class: "ps-2",
                        for: "review-{review.as_variant_name()}",
                        "{review}"
                    }
                }
            }
        }
    }
}
