use crate::api::{login::login, server_working::server_working};
use crate::middleware::message::Config;

use axum::{body::Body, routing::get, Extension, Router};
use tower_http::cors::CorsLayer;

/// Function which is used to combine are the response function under there respective route names
pub fn create_route() -> Router<Body> {
    let data = Config {
        message: "final world".to_owned(),
    };
    Router::new()
        .route("/", get(server_working))
        .route("/login/:id", get(login))
        .layer(Extension(data))
        .layer(CorsLayer::permissive())
}
