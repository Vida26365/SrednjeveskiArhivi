use core::panic;

use dioxus::prelude::*;
use crate::utils::date::Calendar;

// Primer:
// {"date": FormValue(["nqswlknq.sw"]), "title": FormValue(["c7c60e0f-a292-46fb-a5a2-592fe2068203.pdf"]), "keyword": FormValue(["11k\n1ss\n1s1"]), "language": FormValue(["en", "la"]), "main_location": FormValue(["khslqknsw"])}

pub fn parse_input(event: Event<FormData>) {
    let val = event.values();
    println!("{:?}", val);
    let title = &val["title"].as_value();
    let main_location = &val["main_location"].as_value();
    // let keywords_form_value = &val["keyword"].as_value();
    // let keywords = keywords_form_value.lines().collect::<Vec<_>>();
    // let languages : Option<&[String]>= Some(val["language"].as_slice());
    match val.get("language") {
        Some(languages) => {
            let languages = languages.as_slice();
            println!("Languages: {:?}", languages);
        }
        None => println!("No languages found"),
    }
    let date = &val["date"].as_value();
    // let calender =
    let calender = {if &val["calendar"] == "Gregor" {
        Calendar::Gregorian
    } else if &val["calendar"] == "Julijan" {
        Calendar::Julian
    } else {
        panic!("Narobe si napisala ime al pa ostaja še en koledar")
    }};

    println!("Title: {:?}", title);
    println!("Main Location: {:?}", main_location);
    // println!("Keywords: {:?}", keywords);
    // println!("Languages: {:?}", languages);
    println!("Date: {:?}", date);
    println!("Calendar: {:?}", calender);
}
