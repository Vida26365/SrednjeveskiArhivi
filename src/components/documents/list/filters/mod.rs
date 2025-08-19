use dioxus::logger::tracing::debug;
use dioxus::prelude::*;
use sea_orm::sea_query::ConditionType;

use crate::components::documents::list::{
    DocumentFilters,
    DocumentSort,
    DocumentSortColumn,
    DocumentSortOrder,
    FiltersSignal,
    SortSignal,
};
use crate::entities::document::ReviewStatus;
use crate::utils::date::{Calendar, Date};
use crate::utils::language::Language;

mod date;
mod keywords;
mod languages;
mod review;
mod sort;

use date::FilterDate;
use keywords::FilterKeywords;
use languages::FilterLanguages;
use review::FilterReview;
use sort::Sorter;

async fn submit(mut sort: SortSignal, mut filters: FiltersSignal, event: Event<FormData>) {
    debug!("Event: {event:?}");

    let sort: &mut DocumentSort = &mut sort.write();
    let filters: &mut DocumentFilters = &mut filters.write();
    let values = event.values();

    match values["sort-column"].as_value().as_str() {
        "title" => sort.column = DocumentSortColumn::Title,
        "date" => sort.column = DocumentSortColumn::Date,
        "location" => sort.column = DocumentSortColumn::Location,
        "review" => sort.column = DocumentSortColumn::Review,
        _ => {}
    }

    match values["sort-order"].as_value().as_str() {
        "ascending" => sort.order = DocumentSortOrder::Ascending,
        "descending" => sort.order = DocumentSortOrder::Descending,
        _ => {}
    }

    // TODO: Handle errors
    let calendar = Calendar::from_variant_name(&values["calendar"].as_value()).unwrap();

    if values["start-date"].as_value().trim() == "" {
        filters.date.0 = None;
    } else {
        // TODO: Handle errors
        let date = Date::parse(&values["start-date"].as_value(), &calendar).unwrap();
        filters.date.0 = Some(date);
    }

    if values["end-date"].as_value().trim() == "" {
        filters.date.1 = None;
    } else {
        // TODO: Handle errors
        let date = Date::parse(&values["end-date"].as_value(), &calendar).unwrap();
        filters.date.1 = Some(date);
    }

    match values.get("keywords") {
        Some(keywords) => {
            filters.keywords.1 = keywords
                .as_slice()
                .iter()
                .map(|kw| kw.trim())
                .filter(|kw| !kw.is_empty())
                .map(String::from)
                .collect()
        }
        None => filters.keywords.1 = vec![],
    }

    match values.get("keywords-condition") {
        Some(condition) if condition == "on" => filters.keywords.0 = ConditionType::All,
        _ => filters.keywords.0 = ConditionType::Any,
    }

    match values.get("languages") {
        Some(languages) => {
            filters.languages.1 = languages
                .as_slice()
                .iter()
                .filter_map(|lang| Language::from_two_letter_code(lang))
                .collect()
        }
        None => filters.languages.1 = vec![],
    }

    match values.get("languages-condition") {
        Some(condition) if condition == "on" => filters.languages.0 = ConditionType::All,
        _ => filters.languages.0 = ConditionType::Any,
    }

    match values.get("review") {
        Some(review) => {
            filters.review = review
                .as_slice()
                .iter()
                .filter_map(|status| ReviewStatus::from_variant_name(status))
                .collect()
        }
        None => filters.review = vec![],
    }

    debug!("Parsed: {filters:?}");
}

#[component]
pub fn PaneFilters(
    #[props(into)] sort: SortSignal,
    #[props(into)] filters: FiltersSignal,
) -> Element {
    rsx! {
        form {
            onsubmit: move |event| async move {
                submit(sort, filters, event).await;
            },
            ul {
                class: "space-y-4 pb-1",
                li { Sorter {} }
                li { FilterDate {} }
                li { FilterKeywords {} }
                li { FilterLanguages {} }
                li { FilterReview {} }
                li {
                    button {
                        class: "btn btn-soft btn-primary rounded-box",
                        "Uveljavi"
                    }
                }
            }
        }
    }
}
