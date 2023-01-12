use dotenvy::dotenv;
use std::env;

pub async fn run() -> String {
    dotenv().ok();
    let database_uri = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://root:root@localhost/postgres".to_string());
    database_uri
}
