use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "category")]
pub enum Type {
    #[sea_orm(string_value = "text")]
    Text,
    #[sea_orm(string_value = "number")]
    Number,
    #[sea_orm(string_value = "boolean")]
    Boolean,
    #[sea_orm(string_value = "date")]
    Date,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "categories")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(indexed, unique)]
    pub name: String,
    pub description: String,
    pub r#type: Type,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self { id: sea_orm::ActiveValue::Set(Uuid::new_v4()), ..ActiveModelTrait::default() }
    }
}
