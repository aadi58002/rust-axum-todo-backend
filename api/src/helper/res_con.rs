use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub fn res_bad(message: &str) -> Response {
    (StatusCode::BAD_REQUEST, message.to_string()).into_response()
}

pub fn res_good(message: &str) -> Response {
    (StatusCode::ACCEPTED, message.to_string()).into_response()
}

pub fn res_db_fail(message: String) -> Response {
    (
        StatusCode::FAILED_DEPENDENCY,
        format!("Unable to connect to database due to : {message}"),
    )
        .into_response()
}

pub fn res_unauth(message: &str) -> Response {
    (StatusCode::UNAUTHORIZED, message.to_string()).into_response()
}
