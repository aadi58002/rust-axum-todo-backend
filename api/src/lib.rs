mod helper;
mod router;
mod routes;

use database::{get_connection, init_tables};
use router::rounter::create_route;
use std::net::SocketAddr;
use tracing::Level;

pub async fn start_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_line_number(true)
        .with_file(true)
        .with_level(true)
        .with_thread_names(true)
        .with_thread_ids(true)
        .init();
}

#[tokio::main]
pub async fn run() {
    start_tracing().await;
    let db_connection = get_connection().await;

    match init_tables(&db_connection).await {
        Ok(_) => println!("Init tables : Creation complete"),
        Err(e) => println!("Failed due to : {e}"),
    }

    let app = create_route(db_connection);
    let port = 4000;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Server is starting at : {port}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
