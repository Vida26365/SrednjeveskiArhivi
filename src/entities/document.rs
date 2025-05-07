use sea_orm::entity::prelude::*;
use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

use crate::utils::date::Date;
use crate::utils::language::Language;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Languages(pub Vec<Language>);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Keywords(pub Vec<String>);

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
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

    /// The document keywords.
    #[sea_orm(default_value = "[]")]
    pub keywords: Keywords,

    /// Has the document been reviewed?
    #[sea_orm(default_value = false, indexed)]
    pub reviewed: bool,
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
        Relation::Location.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self { id: sea_orm::ActiveValue::Set(Uuid::now_v7()), ..ActiveModelTrait::default() }
    }
}
