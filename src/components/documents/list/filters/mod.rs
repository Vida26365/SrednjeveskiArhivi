use anyhow::{Context, Result, bail};
use dioxus::logger::tracing::debug;
use dioxus::prelude::*;
use sea_orm::sea_query::ConditionType;

use crate::components::alerts::AlertError;
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
mod locations;
mod organizations;
mod persons;
mod review;
mod sort;

use date::FilterDate;
use keywords::FilterKeywords;
use languages::FilterLanguages;
use locations::FilterLocations;
use organizations::FilterOrganizations;
use persons::FilterPersons;
use review::FilterReview;
use sort::Sorter;

fn collect_list_from_form(values: &FormValue) -> Vec<String> {
    values
        .as_slice()
        .iter()
        .map(|kw| kw.trim())
        .filter(|kw| !kw.is_empty())
        .map(String::from)
        .collect()
}

macro_rules! handle_filter_list {
    ($values:expr, $field:expr, $target:expr) => {
        match $values.get($field) {
            Some(val) => $target = collect_list_from_form(val),
            None => $target = vec![],
        }
    };
}

macro_rules! handle_filter_condition {
    ($values:expr, $field:expr, $target:expr) => {
        match $values.get($field) {
            Some(condition) if condition == "on" => $target = ConditionType::All,
            _ => $target = ConditionType::Any,
        }
    };
}

async fn submit(
    mut sort: SortSignal,
    mut filters: FiltersSignal,
    event: Event<FormData>,
) -> Result<()> {
    debug!("Event: {event:?}");

    let sort: &mut DocumentSort = &mut sort.write();
    let filters: &mut DocumentFilters = &mut filters.write();
    let values = event.values();

    // == Sorting

    match values["sort-column"].as_value().as_str() {
        "title" => sort.column = DocumentSortColumn::Title,
        "date" => sort.column = DocumentSortColumn::Date,
        "location" => sort.column = DocumentSortColumn::Location,
        "review" => sort.column = DocumentSortColumn::Review,
        unknown => bail!("Invalid sort column option: {unknown}"),
    }

    match values["sort-order"].as_value().as_str() {
        "ascending" => sort.order = DocumentSortOrder::Ascending,
        "descending" => sort.order = DocumentSortOrder::Descending,
        unknown => bail!("Invalid sort order option: {unknown}"),
    }

    // == Date

    let calendar = &values["calendar"].as_value();
    let calendar = Calendar::from_variant_name(calendar)
        .with_context(|| format!("Invalid calendar variant: {calendar}"))?;

    if values["start-date"].as_value().trim() == "" {
        filters.date.0 = None;
    } else {
        let date = &values["start-date"].as_value();
        let date = Date::parse(date, &calendar)
            .with_context(|| format!("Invalid start date string: {date}"))?;
        filters.date.0 = Some(date);
    }

    if values["end-date"].as_value().trim() == "" {
        filters.date.1 = None;
    } else {
        let date = &values["end-date"].as_value();
        let date = Date::parse(date, &calendar)
            .with_context(|| format!("Invalid end date string: {date}"))?;
        filters.date.1 = Some(date);
    }

    // == Persons

    handle_filter_list!(values, "persons", filters.persons.1);
    handle_filter_condition!(values, "persons-condition", filters.persons.0);

    // == Organizations

    handle_filter_list!(values, "organizations", filters.organizations.1);
    handle_filter_condition!(values, "organizations-condition", filters.organizations.0);

    // == Locations

    handle_filter_list!(values, "locations", filters.locations.1);
    handle_filter_condition!(values, "locations-condition", filters.locations.0);

    // == Keywords

    handle_filter_list!(values, "keywords", filters.keywords.1);
    handle_filter_condition!(values, "keywords-condition", filters.keywords.0);

    // == Languages

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

    handle_filter_condition!(values, "languages-condition", filters.languages.0);

    // == Review

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

    Ok(())
}

#[component]
pub fn PaneFilters(
    #[props(into)] sort: SortSignal,
    #[props(into)] filters: FiltersSignal,
) -> Element {
    let mut state: Signal<Option<Element>> = use_signal(|| None);

    rsx! {
        form {
            onsubmit: move |event| async move {
                match submit(sort, filters, event).await {
                    Ok(_) => state.set(None),
                    Err(error) => state.set(Some(rsx! {
                        AlertError {
                            title: "Napaka pri uveljavljanju filtrov",
                            details: format!("{error:?}"),
                        }
                    })),
                }
            },
            ul {
                class: "space-y-4 [&>li]:space-y-2 pb-1 min-w-fit",
                li { Sorter {} }
                li { FilterDate {} }
                li { FilterPersons {} }
                li { FilterOrganizations {} }
                li { FilterLocations {} }
                li { FilterKeywords {} }
                li { FilterLanguages {} }
                li { FilterReview {} }
                li {
                    button {
                        class: "btn btn-soft btn-primary rounded-box",
                        type: "submit",
                        "Uveljavi"
                    }
                }
                match state() {
                    Some(element) => rsx! { li { { element } } },
                    None => rsx! {},
                }
            }
        }
    }
}
