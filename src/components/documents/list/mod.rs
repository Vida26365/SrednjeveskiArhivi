use dioxus::signals::{ReadOnlySignal as ReadSignal, Signal};
use sea_orm::sea_query::ConditionType;
use sea_orm::{DerivePartialModel, FromQueryResult};
use smart_default::SmartDefault;
use uuid::Uuid;

use crate::entities::document::{Keywords, ReviewStatus};
use crate::entities::{Document, LocationModel};
use crate::utils::date::Date;
use crate::utils::language::Language;

mod filters;
mod table;

pub use filters::PaneFilters;
pub use table::PaneTable;

#[derive(Clone, Debug, PartialEq, Eq, DerivePartialModel, FromQueryResult)]
#[sea_orm(entity = "Document")]
pub struct DocumentResponse {
    /// The document primary key.
    pub id: Uuid,

    /// The document title.
    pub title: String,

    /// The main document date.
    pub date: Option<Date>,

    /// The document location.
    #[sea_orm(skip)]
    pub location: Option<LocationModel>,

    /// The document keywords.
    pub keywords: Keywords,

    /// The document review status.
    pub review: ReviewStatus,
}

pub type DocumentsSignal = ReadSignal<Vec<DocumentResponse>>;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DocumentSortColumn {
    Title,
    Date,
    Location,
    Review,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DocumentSortOrder {
    Ascending,
    Descending,
}

impl From<DocumentSortOrder> for sea_orm::Order {
    fn from(value: DocumentSortOrder) -> Self {
        match value {
            DocumentSortOrder::Ascending => Self::Asc,
            DocumentSortOrder::Descending => Self::Desc,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, SmartDefault)]
pub struct DocumentSort {
    /// The column to sort by.
    #[default(_code = "DocumentSortColumn::Title")]
    pub column: DocumentSortColumn,

    /// The order to sort by.
    #[default(_code = "DocumentSortOrder::Ascending")]
    pub order: DocumentSortOrder,
}

pub type SortSignal = Signal<DocumentSort>;

#[derive(Clone, Debug, PartialEq, Eq, SmartDefault)]
pub struct DocumentFilters {
    /// Filter documents by date range (first, last).
    pub date: (Option<Date>, Option<Date>),

    /// Filter documents by keywords.
    #[default(_code = "(ConditionType::Any, vec![])")]
    pub keywords: (ConditionType, Vec<String>),

    /// Filter documents by languages.
    #[default(_code = "(ConditionType::Any, vec![])")]
    pub languages: (ConditionType, Vec<Language>),

    /// Filter documents by review status.
    pub review: Vec<ReviewStatus>,
}

pub type FiltersSignal = Signal<DocumentFilters>;
