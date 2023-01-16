use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel,Serialize,Deserialize)]
#[sea_orm(table_name = "Users")]
pub struct Model {
    #[sea_orm(primary_key,auto_increment)]
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

impl Related<super::Tasks::Entity> for Entity{
    fn to() -> RelationDef{
        Relation::Task.def()
    }
}


impl ActiveModelBehavior for ActiveModel {}
