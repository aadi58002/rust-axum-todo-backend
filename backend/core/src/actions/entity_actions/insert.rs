use crate::entity::*;
use sea_orm::entity::*;

pub async fn insert_user(
    db_connection: &sea_orm::DatabaseConnection,
    user: User::Model,
) -> Result<User::Model, String> {
    match user.into_active_model().insert(db_connection).await{
        Ok(user) => Ok(user),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn insert_task(
    db_connection: &sea_orm::DatabaseConnection,
    task: Tasks::Model,
) -> Result<Tasks::Model, String> {
    match task.into_active_model().insert(db_connection).await{
        Ok(tasks) => Ok(tasks),
        Err(e) => Err(e.to_string()),
    }
}
