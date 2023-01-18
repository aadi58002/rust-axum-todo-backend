use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::Extension;

pub async fn tasks(
    Extension(db_connection): Extension<core::sea_orm::DatabaseConnection>,
    header: HeaderMap,
) -> impl IntoResponse {}
