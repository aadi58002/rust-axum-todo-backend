use axum::{http::HeaderMap, response::Response};

use super::bad_request::res_bad;

pub fn header_extract(name: &str, header: &HeaderMap) -> Result<String, Response> {
    match header.get(name) {
        Some(name_str) => match name_str.to_str() {
            Ok(val) => Ok(val.to_string()),
            Err(_) => Err(res_bad(&format!("Unable to convet the {} from header to a string", name))),
        },
        None => Err(res_bad(&format!("Unable to get the {} from header", name))),
    }
}
