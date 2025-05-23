#![allow(unused_imports)]

pub mod document;
pub mod document_location;
pub mod document_organization;
pub mod document_person;
pub mod location;
pub mod location_alias;
pub mod organization;
pub mod organization_alias;
pub mod person;
pub mod person_alias;

pub use document::{
    ActiveModel as DocumentActiveModel,
    Entity as Document,
    Model as DocumentModel,
};
pub use document_location::{
    ActiveModel as DocumentLocationActiveModel,
    Entity as DocumentLocation,
    Model as DocumentLocationModel,
};
pub use document_organization::{
    ActiveModel as DocumentOrganizationActiveModel,
    Entity as DocumentOrganization,
    Model as DocumentOrganizationModel,
};
pub use document_person::{
    ActiveModel as DocumentPersonActiveModel,
    Entity as DocumentPerson,
    Model as DocumentPersonModel,
};
pub use location::{
    ActiveModel as LocationActiveModel,
    Entity as Location,
    Model as LocationModel,
};
pub use location_alias::{
    ActiveModel as LocationAliasActiveModel,
    Entity as LocationAlias,
    Model as LocationAliasModel,
};
pub use organization::{
    ActiveModel as OrganizationActiveModel,
    Entity as Organization,
    Model as OrganizationModel,
};
pub use organization_alias::{
    ActiveModel as OrganizationAliasActiveModel,
    Entity as OrganizationAlias,
    Model as OrganizationAliasModel,
};
pub use person::{ActiveModel as PersonActiveModel, Entity as Person, Model as PersonModel};
pub use person_alias::{
    ActiveModel as PersonAliasActiveModel,
    Entity as PersonAlias,
    Model as PersonAliasModel,
};
