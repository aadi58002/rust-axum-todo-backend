use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "Tasks")]
pub struct Model {
    #[sea_orm(primary_key,auto_increment)]
    pub id: i32,
    #[sea_orm(field(not_null))]
    pub username: String,
    #[sea_orm(field(not_null))]
    pub title: String,
    #[sea_orm(field(not_null))]
    pub description: String,
    #[sea_orm(field(not_null))]
    pub status: String,
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
