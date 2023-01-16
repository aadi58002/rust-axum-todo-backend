use crate::routes::{login::login, server_working::server_working};

use axum::{routing::get, Extension, Router};
use tower_http::cors::CorsLayer;

/// Function which is used to combine are the response function under there respective route names
pub fn create_route(db_connection: core::sea_orm::DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(server_working))
        .route("/login", get(login))
        .layer(Extension(db_connection))
        .layer(CorsLayer::permissive())
}
