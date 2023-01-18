use sea_orm::{ActiveEnum, ConnectionTrait, DbErr, EntityTrait, ExecResult, Statement};

pub async fn drop_table<E: EntityTrait>(
    db_connection: &sea_orm::DatabaseConnection,
    entity: E,
) -> Result<ExecResult, DbErr> {
    db_connection
        .execute(Statement::from_sql_and_values(
            db_connection.get_database_backend(),
            &format!(r#"DROP TABLE IF EXISTS "{}""#, entity.table_name()),
            vec![],
        ))
        .await
}

pub async fn drop_type<E: ActiveEnum>(
    db_connection: &sea_orm::DatabaseConnection,
) -> Result<ExecResult, DbErr> {
    db_connection
        .execute(Statement::from_sql_and_values(
            db_connection.get_database_backend(),
            &format!(r#"DROP TYPE IF EXISTS "{}""#, E::name().to_string()),
            vec![],
        ))
        .await
}
