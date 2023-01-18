use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub fn res_bad(message: &str) -> Response
{
    (StatusCode::BAD_REQUEST,message.to_string()).into_response()
}

pub fn res_good
