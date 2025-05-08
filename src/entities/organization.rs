use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "organizations")]
pub struct Model {
    /// The organization primary key.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// The organization name.
    #[sea_orm(indexed)]
    pub name: String,

    /// The organization description.
    #[sea_orm(default_value = "")]
    pub description: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::document::Entity> for Entity {
    fn to() -> RelationDef {
        super::document_organization::Relation::Document.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::document_organization::Relation::Organization.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self { id: sea_orm::ActiveValue::Set(Uuid::now_v7()), ..ActiveModelTrait::default() }
    }
}
