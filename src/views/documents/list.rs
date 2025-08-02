use anyhow::Result;
use dioxus::prelude::*;
use sea_orm::sea_query::{ConditionType, Expr, NullOrdering};
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder};

use crate::components::alerts::AlertError;
use crate::components::documents::list::{DocumentFilters, PaneFilters, PaneTable};
use crate::components::skeleton::Skeleton;
use crate::database::get_database;
use crate::entities::document::DocumentToPrimaryLocation;
use crate::entities::{Document, DocumentColumn};

#[component]
pub fn DocumentList() -> Element {
    let sort_column = use_signal(|| DocumentColumn::Date);
    let sort_order = use_signal(|| Order::Asc);

    let filters = use_signal(DocumentFilters::default);

    let documents: Resource<Result<_>> = use_resource(move || async move {
        let database = get_database().await;

        let mut query = Document::find()
            .find_also_linked(DocumentToPrimaryLocation)
            .order_by_with_nulls(sort_column(), sort_order(), NullOrdering::First);

        let filters: &DocumentFilters = &filters.read();

        match filters.date {
            (Some(start), Some(end)) => {
                query = query.filter(DocumentColumn::Date.between(start, end));
            }
            (Some(start), None) => {
                query = query.filter(DocumentColumn::Date.gte(start));
            }
            (None, Some(end)) => {
                query = query.filter(DocumentColumn::Date.lte(end));
            }
            (None, None) => {}
        }

        if !filters.keywords.1.is_empty() {
            let keywords: Vec<_> = filters.keywords.1.iter().map(|kw| kw.to_lowercase()).collect();
            let placeholders = keywords.iter().map(|_| "?").collect::<Vec<_>>().join(", ");

            match filters.keywords.0 {
                ConditionType::Any => {
                    // At least one keyword in the document matches the filter
                    query = query.filter(Expr::cust_with_values(
                        format!(
                            "EXISTS (
                                SELECT 1 FROM json_each(keywords)
                                WHERE LOWER(value) IN ({placeholders})
                            )"
                        ),
                        keywords,
                    ));
                }
                ConditionType::All => {
                    // All filter keywords are present in the document keywords
                    query = query.filter(Expr::cust_with_values(
                        "NOT EXISTS (
                            SELECT 1 FROM json_each(?) AS fk
                            WHERE LOWER(fk.value) NOT IN (SELECT LOWER(value) FROM json_each(keywords))
                        )",
                        [serde_json::to_string(&keywords)?],
                    ));
                }
            }
        }

        if !filters.languages.1.is_empty() {
            let languages = &filters.languages.1;
            let placeholders = languages.iter().map(|_| "?").collect::<Vec<_>>().join(", ");

            match filters.languages.0 {
                ConditionType::Any => {
                    // At least one language in the document matches the filter
                    query = query.filter(Expr::cust_with_values(
                        format!(
                            "EXISTS (
                                SELECT 1 FROM json_each(languages)
                                WHERE value IN ({placeholders})
                            )"
                        ),
                        languages,
                    ));
                }
                ConditionType::All => {
                    // All filter languages are present in the document languages
                    query = query.filter(Expr::cust_with_values(
                        "NOT EXISTS (
                            SELECT 1 FROM json_each(?) AS fl
                            WHERE fl.value NOT IN (SELECT value FROM json_each(languages))
                        )",
                        [serde_json::to_string(&languages)?],
                    ));
                }
            }
        }

        if !filters.review.is_empty() {
            query = query.filter(DocumentColumn::Review.is_in(filters.review.clone()));
        }

        Ok(query.all(database).await?)
    });

    rsx! {
        link { rel: "stylesheet", href: asset!("/assets/styles/grid.css") }
        script { src: asset!("/assets/scripts/grid.js") }

        div {
            class: "panes",
            div {
                class: "pane p-3",
                "data-default-size": 0.2,
                PaneFilters { filters }
            }
            div {
                class: "pane p-3",
                match &*documents.read_unchecked() {
                    Some(Ok(documents)) => rsx! {
                        PaneTable { documents: documents.clone() }
                    },
                    Some(Err(error)) => rsx! {
                        AlertError {
                            title: "Napaka pri nalaganju dokumentov",
                            details: format!("{error:?}"),
                        }
                    },
                    None => rsx! {
                        Skeleton {}
                    },
                }
            }
        }
    }
}
