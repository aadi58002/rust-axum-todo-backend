use crate::entity::*;
use common::sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};

pub async fn get_user(
    db_connection: &common::sea_orm::DatabaseConnection,
    username: &String,
) -> Result<Option<User::Model>, String> {
    match User::Entity::find()
        .filter(User::Column::Username.contains(&username))
        .one(db_connection)
        .await
    {
        Ok(user) => Ok(user),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn get_all_tasks(
    db_connection: &common::sea_orm::DatabaseConnection,
    user: User::Model,
) -> Result<Vec<Tasks::Model>, String> {
    match user
        .find_related(Tasks::Entity)
        .all(db_connection)
        .await
    {
        Ok(tasks) => Ok(tasks),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn get_task(
    db_connection: &common::sea_orm::DatabaseConnection,
    user: User::Model,
    title: String,
) -> Result<Option<Tasks::Model>, String> {
    match user
        .find_related(Tasks::Entity).filter(Tasks::Column::Title.contains(&title))
        .one(db_connection)
        .await
    {
        Ok(tasks) => Ok(tasks),
        Err(e) => Err(e.to_string()),
    }
}
