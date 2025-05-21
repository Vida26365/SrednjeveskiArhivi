use sea_orm::entity::prelude::*;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Props)]
#[sea_orm(table_name = "locations")]
pub struct Model {
    /// The location primary key.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// The location name.
    #[sea_orm(indexed)]
    pub name: String,

    /// The location description.
    #[sea_orm(default_value = "")]
    pub description: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::location_alias::Entity")]
    Alias,
}

impl Related<super::document::Entity> for Entity {
    fn to() -> RelationDef {
        super::document_location::Relation::Document.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::document_location::Relation::Location.def().rev())
    }
}

impl Related<super::location_alias::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Alias.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self { id: sea_orm::ActiveValue::Set(Uuid::now_v7()), ..ActiveModelTrait::default() }
    }
}
