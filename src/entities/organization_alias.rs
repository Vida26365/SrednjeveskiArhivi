use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "organizations_aliases")]
pub struct Model {
    /// The organization alias primary key.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// The main organization primary key.
    #[sea_orm(indexed, nullable)]
    pub organization: Option<Uuid>,

    /// The organization alias name.
    #[sea_orm(indexed)]
    pub name: String,

    /// The organization alias description.
    #[sea_orm(default_value = "")]
    pub description: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::organization::Entity",
        from = "Column::Organization",
        to = "super::organization::Column::Id"
    )]
    Organization,
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self { id: sea_orm::ActiveValue::Set(Uuid::now_v7()), ..ActiveModelTrait::default() }
    }
}
