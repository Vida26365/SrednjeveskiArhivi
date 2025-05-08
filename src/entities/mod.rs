pub mod document;
pub mod document_location;
pub mod document_organization;
pub mod document_person;
pub mod location;
pub mod organization;
pub mod person;

pub use document::Entity as Document;
pub use document_location::Entity as DocumentLocation;
pub use document_organization::Entity as DocumentOrganization;
pub use document_person::Entity as DocumentPerson;
pub use location::Entity as Location;
pub use organization::Entity as Organization;
pub use person::Entity as Person;
