use crate::routes::{server_working::server_working, user::user, table::table, task::task};

use axum::{routing::get, Extension, Router};
use tower_http::cors::CorsLayer;

/// Function which is used to combine are the response function under there respective route names
pub fn create_route(db_connection: sea_orm::DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(server_working))
        .route("/users", get(user))
        .route("/table", get(table))
        .route("/tasks", get(task))
        .layer(Extension(db_connection))
        .layer(CorsLayer::permissive())
}
