use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "documents_persons")]
pub struct Model {
    /// The referenced document ID.
    #[sea_orm(primary_key, auto_increment = false)]
    pub document: Uuid,

    /// The referenced person ID.
    #[sea_orm(primary_key, auto_increment = false)]
    pub person: Uuid,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::document::Entity",
        from = "Column::Document",
        to = "super::document::Column::Id"
    )]
    Document,

    #[sea_orm(
        belongs_to = "super::person::Entity",
        from = "Column::Person",
        to = "super::person::Column::Id"
    )]
    Person,
}

impl ActiveModelBehavior for ActiveModel {}
