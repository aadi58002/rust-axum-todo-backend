use core::entity_actions::save::save_entity;

use common::{axum::{http::HeaderMap, response::IntoResponse, Extension}, sea_orm::IntoActiveModel};

use crate::helper::{header_extract::header_extract, res_con::{res_good, res_db_fail}};

// Function to get the headers from the request
pub async fn register(Extension(db_connection): Extension<common::sea_orm::DatabaseConnection>,header: HeaderMap) -> impl IntoResponse{
    let username = match header_extract("username", &header) {
        Ok(val) => val,
        Err(e) => return e,
    };
    let email = match header_extract("email", &header) {
        Ok(val) => val,
        Err(e) => return e,
    };
    let password = match header_extract("password", &header) {
        Ok(val) => val,
        Err(e) => return e,
    };
    match save_entity::<core::User::ActiveModel>(&db_connection, core::User::Model::new(username,email,password).into_active_model()).await{
        Ok(_) => res_good("Registration was successful"),
        Err(e) => res_db_fail(e),
    }
}
