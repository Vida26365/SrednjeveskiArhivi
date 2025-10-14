use anyhow::Result;
use dioxus::prelude::*;
use sea_orm::sea_query::{Condition, ConditionType, Expr, ExprTrait, Func};
use sea_orm::{
    ColumnTrait,
    EntityTrait,
    JoinType,
    QueryFilter,
    QueryOrder,
    QuerySelect,
    QueryTrait,
    RelationTrait,
};

use crate::components::alerts::AlertError;
use crate::components::documents::list::{
    DocumentFilters,
    DocumentResponse,
    DocumentSearch,
    DocumentSort,
    DocumentSortColumn,
    PaneFilters,
    PaneTable,
};
use crate::components::skeleton::Skeleton;
use crate::database::get_database;
use crate::entities::{Document, DocumentColumn, DocumentRelation, LocationColumn};

macro_rules! generate_entity_name_filters {
    ($query:ident, $filter:expr, $base:ident $(, primary = $primary:ident)?) => {
        if !$filter.1.is_empty() {
            let names: Vec<String> = $filter.1.iter().map(|it| it.to_lowercase()).collect();

            // Any: At least one name in the document matches the filter
            // All: All filter names are present in the document names
            let groups: Vec<Vec<String>> = match $filter.0 {
                ConditionType::Any => vec![names],
                ConditionType::All => names.into_iter().map(|name| vec![name]).collect(),
            };

            for group in groups {
                paste::paste! {
                    // Find entity IDs that have the main name matching the filter
                    let ids_from_names_subquery = crate::entities::[<$base>]::find()
                        .select_only()
                        .column(crate::entities::[<$base Column>]::Id)
                        .filter(Func::lower(Expr::col(crate::entities::[<$base Column>]::Name)).is_in(&group))
                        .into_query();

                    // Find entity IDs that have an alias matching the filter
                    let ids_from_aliases_subquery = crate::entities::[<$base Alias>]::find()
                        .select_only()
                        .column(crate::entities::[<$base AliasColumn>]::[<$base>])
                        .filter(Func::lower(Expr::col(crate::entities::[<$base AliasColumn>]::Name)).is_in(&group))
                        .into_query();

                    // Base expression that will be combined with the below expressions
                    let mut combined_filter_condition = Condition::any();

                    $(
                        let primary_entity_expression =
                            Expr::col((crate::entities::Document, DocumentColumn::$primary)).in_subquery(ids_from_names_subquery.clone())
                            .or(Expr::col((crate::entities::Document, DocumentColumn::$primary)).in_subquery(ids_from_aliases_subquery.clone()));
                        combined_filter_condition = combined_filter_condition.add(primary_entity_expression);
                    )?

                    let related_entity_expression =
                        Expr::col(crate::entities::[<Document $base Column>]::[<$base>]).in_subquery(ids_from_names_subquery)
                        .or(Expr::col(crate::entities::[<Document $base Column>]::[<$base>]).in_subquery(ids_from_aliases_subquery));
                    combined_filter_condition = combined_filter_condition.add(related_entity_expression);

                    $query = $query.filter(
                        Expr::col((crate::entities::Document, crate::entities::DocumentColumn::Id)).in_subquery(
                            crate::entities::[<Document $base>]::find()
                                .select_only()
                                .column(crate::entities::[<Document $base Column>]::Document)
                                .filter(combined_filter_condition)
                                .into_query(),
                        ),
                    );
                }
            }
        }
    };
}

fn apply_sorting(
    mut query: sea_orm::Select<Document>,
    sort: &DocumentSort,
) -> Result<sea_orm::Select<Document>> {
    match sort.column {
        DocumentSortColumn::Title => {
            query = query.order_by(DocumentColumn::Title, sort.order.into());
        }
        DocumentSortColumn::Date => {
            query = query.order_by(DocumentColumn::Date, sort.order.into());
        }
        DocumentSortColumn::Location => {
            query = query.order_by(LocationColumn::Name, sort.order.into());
        }
        DocumentSortColumn::Review => {
            query = query.order_by(DocumentColumn::Review, sort.order.into());
        }
    }

    Ok(query)
}

fn apply_filtering(
    mut query: sea_orm::Select<Document>,
    filters: &DocumentFilters,
) -> Result<sea_orm::Select<Document>> {
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

    generate_entity_name_filters!(query, filters.persons, Person);
    generate_entity_name_filters!(query, filters.organizations, Organization);
    generate_entity_name_filters!(query, filters.locations, Location, primary = Location);

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
        query = query.filter(DocumentColumn::Review.is_in(filters.review.iter().copied()));
    }

    Ok(query)
}

fn apply_search(
    mut query: sea_orm::Select<Document>,
    search: &DocumentSearch,
) -> Result<sea_orm::Select<Document>> {
    let text = search.query.trim();

    if !text.is_empty() {
        let expr = "documents.id IN (SELECT id FROM documents_fts(?))";
        query = query.filter(Expr::cust_with_values(expr, [text.to_string()]));
    }
    Ok(query)
}

#[component]
pub fn DocumentList() -> Element {
    let sort = use_signal(DocumentSort::default);
    let filters = use_signal(DocumentFilters::default);
    let search = use_signal(DocumentSearch::default);

    let documents: Resource<Result<Vec<DocumentResponse>>> = use_resource(move || async move {
        let database = get_database().await;

        let mut query = Document::find().join(JoinType::LeftJoin, DocumentRelation::Location.def());

        query = apply_sorting(query, &sort.read())?;
        query = apply_filtering(query, &filters.read())?;
        query = apply_search(query, &search.read())?;

        Ok(query.into_partial_model().all(database).await?)
    });

    rsx! {
        link { rel: "stylesheet", href: asset!("/assets/styles/grid.css") }
        script { src: asset!("/assets/scripts/grid.js") }

        div {
            class: "panes",
            div {
                class: "pane p-3",
                "data-default-size": 0.2,
                PaneFilters { sort, filters }
            }
            div {
                class: "pane p-3",
                match &*documents.read_unchecked() {
                    Some(Ok(documents)) => rsx! {
                        PaneTable { documents: documents.clone(), search }
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
