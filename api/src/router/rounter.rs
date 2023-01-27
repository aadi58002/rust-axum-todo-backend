use crate::routes::{server_working::server_working, user::users, table::table, tasks::tasks};

use common::axum::{routing::get, Extension, Router};
use tower_http::cors::CorsLayer;

/// Function which is used to combine are the response function under there respective route names
pub fn create_route(db_connection: common::sea_orm::DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(server_working))
        .route("/users", get(users))
        .route("/table", get(table))
        .route("/tasks", get(tasks))
        .layer(Extension(db_connection))
        .layer(CorsLayer::permissive())
}
