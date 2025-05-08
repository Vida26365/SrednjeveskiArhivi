use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "documents_locations")]
pub struct Model {
    /// The referenced document ID.
    #[sea_orm(primary_key, auto_increment = false)]
    pub document: Uuid,

    /// The referenced location ID.
    #[sea_orm(primary_key, auto_increment = false)]
    pub location: Uuid,
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
        belongs_to = "super::location::Entity",
        from = "Column::Location",
        to = "super::location::Column::Id"
    )]
    Location,
}

impl ActiveModelBehavior for ActiveModel {}
