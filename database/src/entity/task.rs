use sea_orm as sea_orm;
use sea_orm::entity::prelude::*;
use serde::{self,Serialize, Deserialize};

#[derive(Clone,Default, Debug, PartialEq, DeriveEntityModel,Serialize,Deserialize)]
#[serde(crate = "self::serde")]
#[sea_orm(table_name = "Tasks")]
pub struct Model {
    #[sea_orm(primary_key,auto_increment)]
    #[serde(skip_serializing,skip_deserializing)]
    pub id: i32,
    pub username: String,
    #[sea_orm(primary_key)]
    pub title: String,
    pub description: Option<String>,
    pub status: TaskState,
}

#[derive(Default,Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum,Serialize,Deserialize)]
#[serde(crate = "self::serde")]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "taskstate")]
pub enum TaskState{
    #[sea_orm(string_value = "C")]
    Completed,
    #[sea_orm(string_value = "P")]
    #[default]
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

impl Model {
    pub fn new(username: String, title: String, description: Option<String>) -> Self {
        Self {
            username,
            title,
            description,
             ..Default::default()
        }
    }
}
