use sea_orm;
use sea_orm::entity::prelude::*;
use serde::{self,Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel,Serialize,Deserialize)]
#[serde(crate = "self::serde")]
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
    #[sea_orm(has_many = "super::Task::Entity")]
    Task,
}

impl Related<super::Task::Entity> for Entity {
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
