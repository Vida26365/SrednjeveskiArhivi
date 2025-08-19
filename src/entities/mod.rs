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
    Column as DocumentColumn,
    Entity as Document,
    Model as DocumentModel,
    Relation as DocumentRelation,
};
pub use document_location::{
    ActiveModel as DocumentLocationActiveModel,
    Column as DocumentLocationColumn,
    Entity as DocumentLocation,
    Model as DocumentLocationModel,
    Relation as DocumentLocationRelation,
};
pub use document_organization::{
    ActiveModel as DocumentOrganizationActiveModel,
    Column as DocumentOrganizationColumn,
    Entity as DocumentOrganization,
    Model as DocumentOrganizationModel,
    Relation as DocumentOrganizationRelation,
};
pub use document_person::{
    ActiveModel as DocumentPersonActiveModel,
    Column as DocumentPersonColumn,
    Entity as DocumentPerson,
    Model as DocumentPersonModel,
    Relation as DocumentPersonRelation,
};
pub use location::{
    ActiveModel as LocationActiveModel,
    Column as LocationColumn,
    Entity as Location,
    Model as LocationModel,
    Relation as LocationRelation,
};
pub use location_alias::{
    ActiveModel as LocationAliasActiveModel,
    Column as LocationAliasColumn,
    Entity as LocationAlias,
    Model as LocationAliasModel,
    Relation as LocationAliasRelation,
};
pub use organization::{
    ActiveModel as OrganizationActiveModel,
    Column as OrganizationColumn,
    Entity as Organization,
    Model as OrganizationModel,
    Relation as OrganizationRelation,
};
pub use organization_alias::{
    ActiveModel as OrganizationAliasActiveModel,
    Column as OrganizationAliasColumn,
    Entity as OrganizationAlias,
    Model as OrganizationAliasModel,
    Relation as OrganizationAliasRelation,
};
pub use person::{
    ActiveModel as PersonActiveModel,
    Column as PersonColumn,
    Entity as Person,
    Model as PersonModel,
    Relation as PersonRelation,
};
pub use person_alias::{
    ActiveModel as PersonAliasActiveModel,
    Column as PersonAliasColumn,
    Entity as PersonAlias,
    Model as PersonAliasModel,
    Relation as PersonAliasRelation,
};
