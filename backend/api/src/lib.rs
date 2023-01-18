mod routes;
mod middleware;
mod router;
mod helper;

use router::rounter::create_route;
use std::net::SocketAddr;
use core::{get_connection,init_tables};

pub async fn start_tracing() {
    tracing_subscriber::fmt::init();
}
#[tokio::main]
pub async fn run() {
    let db_connection = get_connection().await;

    match init_tables(&db_connection).await{
        Ok(_) => println!("Init tables : Creation complete"),
        Err(e) => println!("Failed due to : {}", e.to_string()),
    }

    let app = create_route(db_connection);
    let port = 4000;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Server is starting at : {}", port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
