use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "labels_date")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub document: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub category: Uuid,
    pub value: Date,
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
        belongs_to = "super::category::Entity",
        from = "Column::Category",
        to = "super::category::Column::Id"
    )]
    Category,
}

impl ActiveModelBehavior for ActiveModel {}
