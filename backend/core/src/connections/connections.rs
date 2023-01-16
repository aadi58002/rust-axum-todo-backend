use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{time::Duration, env};


pub async fn get_connection() -> DatabaseConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://root:root@localhost/postgres".to_string());
    let mut connect = ConnectOptions::new(database_url);
    connect
        .connect_timeout(Duration::from_secs(3))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug);
    Database::connect(connect)
        .await
        .expect("Unable to connect to the database")
}
