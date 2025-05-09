use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "locations_aliases")]
pub struct Model {
    /// The location alias primary key.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// The main location primary key.
    #[sea_orm(indexed, nullable)]
    pub location: Option<Uuid>,

    /// The location alias name.
    #[sea_orm(indexed)]
    pub name: String,

    /// The location alias description.
    #[sea_orm(default_value = "")]
    pub description: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::location::Entity",
        from = "Column::Location",
        to = "super::location::Column::Id"
    )]
    Location,
}

impl Related<super::location::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Location.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self { id: sea_orm::ActiveValue::Set(Uuid::now_v7()), ..ActiveModelTrait::default() }
    }
}
