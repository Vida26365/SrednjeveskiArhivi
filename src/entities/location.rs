use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
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
pub enum Relation {}

impl Related<super::document::Entity> for Entity {
    fn to() -> RelationDef {
        super::document_location::Relation::Document.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::document_location::Relation::Location.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self { id: sea_orm::ActiveValue::Set(Uuid::now_v7()), ..ActiveModelTrait::default() }
    }
}
