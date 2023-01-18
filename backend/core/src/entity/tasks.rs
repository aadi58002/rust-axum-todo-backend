use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel,Serialize,Deserialize)]
#[sea_orm(table_name = "Tasks")]
pub struct Model {
    #[sea_orm(primary_key,auto_increment)]
    pub id: i32,
    #[sea_orm(unique)]
    pub username: String,
    pub title: String,
    pub description: String,
    pub status: TaskState,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum,Serialize,Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "taskstate")]
pub enum TaskState{
    #[sea_orm(string_value = "C")]
    Completed,
    #[sea_orm(string_value = "P")]
    Pending
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::User::Entity",
        from = "Column::Username",
        to = "super::User::Column::Username",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::User::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
