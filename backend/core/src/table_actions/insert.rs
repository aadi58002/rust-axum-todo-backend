use sea_orm::{ActiveModelBehavior, ActiveModelTrait, EntityTrait, IntoActiveModel};

pub async fn insert_in_table<E: ActiveModelBehavior + ActiveModelTrait + EntityTrait>(
    db_connection: &sea_orm::DatabaseConnection,
    active_entity: E,
) -> Result<<<E as ActiveModelTrait>::Entity as EntityTrait>::Model, sea_orm::DbErr>
where
    <<E as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<E>,
{
    active_entity.insert(db_connection).await
}
