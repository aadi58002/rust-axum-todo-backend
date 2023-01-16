use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "Users")]
pub struct Model {
    #[sea_orm(primary_key,auto_increment)]
    pub id: i32,
    #[sea_orm(field(not_null))]
    pub username: String,
    #[sea_orm(field(not_null))]
    pub email: String,
    #[sea_orm(field(not_null))]
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
