use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "persons")]
pub struct Model {
    /// The person primary key.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// The person name.
    #[sea_orm(indexed)]
    pub name: String,

    /// The person description.
    #[sea_orm(default_value = "")]
    pub description: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::person_alias::Entity")]
    Alias,
}

impl Related<super::document::Entity> for Entity {
    fn to() -> RelationDef {
        super::document_person::Relation::Document.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::document_person::Relation::Person.def().rev())
    }
}

impl Related<super::person_alias::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Alias.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self { id: sea_orm::ActiveValue::Set(Uuid::now_v7()), ..ActiveModelTrait::default() }
    }
}
