use dioxus::signals::{ReadOnlySignal as ReadSignal, Signal};
use sea_orm::sea_query::ConditionType;
use smart_default::SmartDefault;

use crate::entities::document::ReviewStatus;
use crate::entities::{DocumentModel, LocationModel};
use crate::utils::date::Date;
use crate::utils::language::Language;

mod filters;
mod table;

pub use filters::PaneFilters;
pub use table::PaneTable;

pub type DocumentsSignal = ReadSignal<Vec<(DocumentModel, Option<LocationModel>)>>;

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
