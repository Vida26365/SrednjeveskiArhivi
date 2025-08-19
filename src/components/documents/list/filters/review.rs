use dioxus::prelude::*;
use sea_orm::Iterable;

use crate::entities::document::ReviewStatus;

#[component]
pub fn FilterReview() -> Element {
    rsx! {
        label {
            class: "flex pb-2 font-semibold",
            "Stanje"
        }
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
