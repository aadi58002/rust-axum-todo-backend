use common::sea_orm::{ConnectionTrait, DbErr, EntityTrait, ExecResult, Statement};

pub async fn clear_table<E: EntityTrait>(
    db_connection: &common::sea_orm::DatabaseConnection,
    entity: E,
) -> Result<ExecResult, DbErr> {
    db_connection
        .execute(Statement::from_sql_and_values(
            db_connection.get_database_backend(),
            &format!(r#"TRUNCATE TABLE "{}""#, entity.table_name()),
            vec![],
        ))
        .await
}
