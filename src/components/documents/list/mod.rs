use dioxus::signals::ReadOnlySignal as ReadSignal;

use crate::entities::{DocumentModel, LocationModel};

mod filters;
mod table;

pub use filters::PaneFilters;
pub use table::PaneTable;

pub type DocumentsSignal = ReadSignal<Vec<(DocumentModel, Option<LocationModel>)>>;
