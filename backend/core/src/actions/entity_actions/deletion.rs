use sea_orm::{ActiveModelBehavior, DeleteResult, ModelTrait};

pub async fn delete_enitity<E>(
    db_connection: &sea_orm::DatabaseConnection,
    entity: E,
) -> Result<DeleteResult, String>
where
    E: ModelTrait + ActiveModelBehavior,
{
    match sea_orm::ActiveModelTrait::delete(entity, db_connection).await {
        Ok(val) => Ok(val),
        Err(e) => Err(e.to_string()),
    }
}
