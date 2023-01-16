use sea_orm::{ConnectionTrait, EntityTrait, Statement};

pub async fn drop_table<E>(db_connection: &sea_orm::DatabaseConnection, entity: E)
where
    E: EntityTrait,
{
    println!(
        "{:?}",
        Statement::from_sql_and_values(
            db_connection.get_database_backend(),
            &format!(r#"DROP TABLE {}"#, entity.table_name()),
            vec![],
        )
    );
    let execution = db_connection.execute(Statement::from_sql_and_values(
        db_connection.get_database_backend(),
        &format!(r#"DROP TABLE {}"#, entity.table_name()),
        vec![],
    ));
    
    match execution.await {
        Ok(_) => println!("Deleted {}", entity.table_name()),
        Err(e) => println!("Error: {}", e),
    }
}
