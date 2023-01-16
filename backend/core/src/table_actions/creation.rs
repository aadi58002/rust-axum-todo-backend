use sea_orm::{Schema, ConnectionTrait, EntityTrait};

pub async fn create_table<E>(db_connection: &sea_orm::DatabaseConnection, entity: E)
where E: EntityTrait{
    let backend = db_connection.get_database_backend();
    let schema = Schema::new(backend);
    let execution = db_connection.execute(backend.build(&schema.create_table_from_entity(entity)));
    match execution.await {
        Ok(_) => println!("Created {}", entity.table_name()),
        Err(e) => println!("Error: {}", e),
    }
}
