use sea_orm::entity::*;

pub async fn insert_entity<E>(
    db_connection: &sea_orm::DatabaseConnection,
    entity: E,
) -> Result<<<E as ActiveModelTrait>::Entity as EntityTrait>::Model, String>
where
    E: ActiveModelBehavior + std::marker::Send,
    <<E as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model:
        sea_orm::IntoActiveModel<E>,
{
    println!("{:?}",entity);
    match entity.insert(db_connection).await {
        Ok(val) => Ok(val),
        Err(e) => Err(e.to_string()),
    }
}
