use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "category")]
pub enum Type {
    #[sea_orm(string_value = "string")]
    String,
    #[sea_orm(string_value = "integer")]
    Integer,
    #[sea_orm(string_value = "float")]
    Float,
    #[sea_orm(string_value = "boolean")]
    Boolean,
    #[sea_orm(string_value = "date")]
    Date,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "categories")]
pub struct Model {
    /// The category identifier.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// The category identifier.
    #[sea_orm(indexed, unique)]
    pub name: String,

    /// The category description (optional).
    #[sea_orm(nullable)]
    pub description: Option<String>,

    /// The category type.
    pub r#type: Type,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self { id: sea_orm::ActiveValue::Set(Uuid::now_v7()), ..ActiveModelTrait::default() }
    }
}
