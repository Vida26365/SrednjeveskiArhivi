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

pub use document::Entity as Document;
pub use document_location::Entity as DocumentLocation;
pub use document_organization::Entity as DocumentOrganization;
pub use document_person::Entity as DocumentPerson;
pub use location::Entity as Location;
pub use location_alias::Entity as LocationAlias;
pub use organization::Entity as Organization;
pub use organization_alias::Entity as OrganizationAlias;
pub use person::Entity as Person;
pub use person_alias::Entity as PersonAlias;
