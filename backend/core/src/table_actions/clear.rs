use sea_orm::{ConnectionTrait, EntityTrait, Statement};

pub async fn clear_table<E>(db_connection: &sea_orm::DatabaseConnection, entity: E)
where
    E: EntityTrait,
{
    let execution = db_connection.execute(Statement::from_sql_and_values(
        db_connection.get_database_backend(),
        &format!(r#"TRUNCATE TABLE "{}""#, entity.table_name()),
        vec![],
    ));

    match execution.await {
        Ok(_) => println!("Cleared {}", entity.table_name()),
        Err(e) => println!("Error: {}", e),
    }
}
