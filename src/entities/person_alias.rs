use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "persons_aliases")]
pub struct Model {
    /// The person alias primary key.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// The main person primary key.
    #[sea_orm(indexed, nullable)]
    pub person: Option<Uuid>,

    /// The person alias name.
    #[sea_orm(indexed)]
    pub name: String,

    /// The person alias description.
    #[sea_orm(default_value = "")]
    pub description: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::person::Entity",
        from = "Column::Person",
        to = "super::person::Column::Id"
    )]
    Person,
}

impl Related<super::person::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Person.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self { id: sea_orm::ActiveValue::Set(Uuid::now_v7()), ..ActiveModelTrait::default() }
    }
}
