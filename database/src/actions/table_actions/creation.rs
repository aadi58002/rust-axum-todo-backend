use sea_orm::{ActiveEnum, ConnectionTrait, DbErr, EntityTrait, ExecResult, Schema};

pub async fn create_table<E: EntityTrait>(
    db_connection: &sea_orm::DatabaseConnection,
    entity: E,
) -> Result<ExecResult, DbErr> {
    let backend = db_connection.get_database_backend();
    let schema = Schema::new(backend);
    db_connection
        .execute(backend.build(&schema.create_table_from_entity(entity)))
        .await
}

pub async fn create_type<E: ActiveEnum>(
    db_connection: &sea_orm::DatabaseConnection,
) -> Result<ExecResult, DbErr> {
    let backend = db_connection.get_database_backend();
    let schema = Schema::new(backend);
    db_connection
        .execute(backend.build(&schema.create_enum_from_active_enum::<E>()))
        .await
}
