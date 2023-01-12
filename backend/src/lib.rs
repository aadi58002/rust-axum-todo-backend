mod api;
mod middleware;
mod routes;

use routes::create_route;
use std::net::SocketAddr;

pub async fn start_tracing() {
    tracing_subscriber::fmt::init();
}

pub async fn run() {
    let app = create_route();
    let port = 4000;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Server is starting at : {}", port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
