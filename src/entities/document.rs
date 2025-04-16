use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "documents")]
pub struct Model {
    /// The document identifier.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// The document filename.
    pub filename: String,

    /// The document title.
    pub title: String,

    /// The document summary (optional).
    #[sea_orm(nullable)]
    pub summary: Option<String>,

    /// The document content (optional).
    #[sea_orm(nullable)]
    pub content: Option<String>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self { id: sea_orm::ActiveValue::Set(Uuid::new_v4()), ..ActiveModelTrait::default() }
    }
}
