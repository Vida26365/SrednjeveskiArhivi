use dioxus::logger::tracing::info;
// use dioxus::logger::tracing::Event;
use dioxus::prelude::*;
use sea_orm::sea_query::Keyword;

// Primer:
// {"date": FormValue(["nqswlknq.sw"]), "title": FormValue(["c7c60e0f-a292-46fb-a5a2-592fe2068203.pdf"]), "keyword": FormValue(["11k\n1ss\n1s1"]), "language": FormValue(["en", "la"]), "main_location": FormValue(["khslqknsw"])}

pub fn parse_input(event: Event<FormData>) {
    let val = event.values();
    println!("{:?}", val);
    let title = &val["title"].as_value();
    let main_location = &val["main_location"].as_value();
    let keywords_form_value = &val["keyword"].as_value();
    let keywords = keywords_form_value.lines().collect::<Vec<_>>();
    let languages = val["language"].as_slice();
    let date = &val["date"].as_value();
}
