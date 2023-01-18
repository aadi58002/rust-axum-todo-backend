use sea_orm::entity::{*,prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "Users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment)]
    pub id: i32,
    #[sea_orm(unique)]
    pub username: String,
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

impl ActiveModel {
    pub fn new(username: String, email: String, password: String) -> Self {
        ActiveModel {
            id: Default::default(),
            username: Set(username),
            email: Set(email),
            password: Set(password),
        }
    }
}
