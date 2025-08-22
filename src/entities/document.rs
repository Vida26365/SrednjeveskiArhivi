use std::fmt::{Display, Formatter};

use dioxus::prelude::*;
use sea_orm::entity::prelude::*;
use sea_orm::{FromJsonQueryResult, LinkDef};
use serde::{Deserialize, Serialize};

use crate::utils::date::Date;
use crate::utils::language::Language;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Languages(pub Vec<Language>);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Persons(pub Vec<String>);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Keywords(pub Vec<String>);

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "status")]
pub enum ReviewStatus {
    /// The document is not reviewed.
    #[sea_orm(string_value = "NOT_REVIEWED")]
    NotReviewed,

    /// The document is under review.
    #[sea_orm(string_value = "UNDER_REVIEW")]
    UnderReview,

    /// The document is reviewed.
    #[sea_orm(string_value = "REVIEWED")]
    Reviewed,
}

impl ReviewStatus {
    pub fn as_variant_name(&self) -> &'static str {
        match self {
            ReviewStatus::NotReviewed => "not-reviewed",
            ReviewStatus::UnderReview => "under-review",
            ReviewStatus::Reviewed => "reviewed",
        }
    }
}

impl ReviewStatus {
    pub fn from_variant_name(name: &str) -> Option<Self> {
        match name {
            "not-reviewed" => Some(ReviewStatus::NotReviewed),
            "under-review" => Some(ReviewStatus::UnderReview),
            "reviewed" => Some(ReviewStatus::Reviewed),
            _ => None,
        }
    }
}

impl Display for ReviewStatus {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReviewStatus::NotReviewed => write!(fmt, "Ni pregledan"),
            ReviewStatus::UnderReview => write!(fmt, "V pregledu"),
            ReviewStatus::Reviewed => write!(fmt, "Pregledan"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Props)]
#[sea_orm(table_name = "documents")]
pub struct Model {
    /// The document primary key.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// The document filename.
    #[sea_orm(indexed)]
    pub filename: String,

    /// The document title.
    #[sea_orm(indexed)]
    pub title: String,

    /// The document summary.
    #[sea_orm(default_value = "")]
    pub summary: String,

    /// The document metadata.
    #[sea_orm(default_value = "")]
    pub metadata: String,

    /// The document content.
    #[sea_orm(default_value = "")]
    pub content: String,

    /// The main document date.
    #[sea_orm(indexed, nullable)]
    pub date: Option<Date>,

    /// The main document location.
    #[sea_orm(indexed, nullable)]
    pub location: Option<Uuid>,

    /// The document languages.
    #[sea_orm(default_value = "[]")]
    pub languages: Languages,

    // /// The document persons.
    // #[sea_orm(default_value = "[]")]
    // pub persons: Persons,

    /// The document keywords.
    #[sea_orm(default_value = "[]")]
    pub keywords: Keywords,

    /// The document review status.
    #[sea_orm(default_value = "NOT_REVIEWED", indexed)]
    pub review: ReviewStatus,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::location::Entity",
        from = "Column::Location",
        to = "super::location::Column::Id"
    )]
    Location,
}

impl Related<super::location::Entity> for Entity {
    fn to() -> RelationDef {
        super::document_location::Relation::Location.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::document_location::Relation::Document.def().rev())
    }
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        super::document_organization::Relation::Organization.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::document_organization::Relation::Document.def().rev())
    }
}

impl Related<super::person::Entity> for Entity {
    fn to() -> RelationDef {
        super::document_person::Relation::Person.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::document_person::Relation::Document.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self { id: sea_orm::ActiveValue::Set(Uuid::now_v7()), ..ActiveModelTrait::default() }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DocumentToPrimaryLocation;

impl Linked for DocumentToPrimaryLocation {
    type FromEntity = Entity;
    type ToEntity = super::location::Entity;

    fn link(&self) -> Vec<LinkDef> {
        vec![Relation::Location.def()]
    }
}
