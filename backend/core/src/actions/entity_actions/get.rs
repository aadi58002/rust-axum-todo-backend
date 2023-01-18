use crate::entity::*;
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};

pub async fn get_user(
    db_connection: &sea_orm::DatabaseConnection,
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

pub async fn get_tasks(
    db_connection: &sea_orm::DatabaseConnection,
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
