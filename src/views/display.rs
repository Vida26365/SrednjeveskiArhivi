use dioxus::prelude::*;
use std::fs;

#[component]
pub fn Display() -> Element {
    // let spremnjkw = "tekst";
    // let tekst = asset
    let contents = fs::read_to_string(r"tekst_datoteke\tekst.txt")
        .expect("Should have been able to read the file");
    rsx! {
        div {
            div {img {src : asset!("zapisi/GZL I-1 (1243 april 13)-1.jpg")}}
            div {span {"{contents}"}}
        }
    }
}
