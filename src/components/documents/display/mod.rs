use dioxus::signals::ReadOnlySignal as ReadSignal;

use crate::entities::{
    DocumentModel,
    LocationAliasModel,
    LocationModel,
    OrganizationAliasModel,
    OrganizationModel,
    PersonAliasModel,
    PersonModel,
};

mod pdf;
mod properties;
mod text;

pub use pdf::PanePdf;
pub use properties::PaneProperties;
pub use text::PaneText;

pub type DocumentSignal = ReadSignal<DocumentModel>;
pub type PrimaryLocationSignal = ReadSignal<Option<LocationModel>>;
pub type LocationsSignal = ReadSignal<Vec<(LocationModel, Vec<LocationAliasModel>)>>;
pub type OrganizationsSignal = ReadSignal<Vec<(OrganizationModel, Vec<OrganizationAliasModel>)>>;
pub type PersonsSignal = ReadSignal<Vec<(PersonModel, Vec<PersonAliasModel>)>>;
