use core::entity_actions::get::get_user;

use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::Extension;

use crate::helper::res_con::{res_bad, res_good, res_db_fail, res_unauth};
use crate::helper::header_extract::*;

pub async fn login(
    Extension(db_connection): Extension<core::sea_orm::DatabaseConnection>,
    header: HeaderMap,
) -> impl IntoResponse {
    let username = match header_extract("username", &header) {
        Ok(val) => val,
        Err(e) => return e,
    };
    let password = match header_extract("password", &header) {
        Ok(val) => val,
        Err(e) => return e,
    };
    match get_user(&db_connection, &username).await{
        Ok(user_option) => {
            match user_option {
                Some(user) => {
                    if user.password == password{
                        res_good("Successful login")
                    }else{
                        res_unauth("login failed with wrong password")
                    }
                },
                None => res_bad(&format!("No user with username '{}' was found",username)),
            }
        },
        Err(e) => res_db_fail(e),
    }
}
