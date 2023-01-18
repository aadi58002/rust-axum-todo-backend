use axum::{http::HeaderMap, response::IntoResponse};

// Function to get the headers from the request
pub async fn register(headers: HeaderMap) -> impl IntoResponse{
    headers
}
