use common::sea_orm;
use common::sea_orm::entity::prelude::*;
use common::serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel,Serialize,Deserialize)]
#[sea_orm(table_name = "Users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    #[serde(skip_serializing,skip_deserializing)]
    pub id: i32,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::Tasks::Entity")]
    Task,
}

impl Related<super::Tasks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn new(username: String, email: String, password: String) -> Self {
        Self {
            id: Default::default(),
            username,
            email,
            password,
        }
    }
}
