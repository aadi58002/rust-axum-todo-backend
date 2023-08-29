use crate::helper::{
    header_extract::*,
    res_con::{res_bad, res_db_fail, res_good, res_unauth},
};
use axum::{http::HeaderMap, response::IntoResponse, Extension};
use database::entity_actions::{
    deletion::delete_enitity, get::get_user, insert::insert_entity, update::update_entity,
};
use sea_orm::{entity::*, IntoActiveModel};
use serde_json;

pub async fn user(
    Extension(db_connection): Extension<sea_orm::DatabaseConnection>,
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
    let action = match header_extract("action", &header) {
        Ok(val) => val,
        Err(e) => return e,
    };
    match action.as_str() {
        "register" => {
            let email = match header_extract("email", &header) {
                Ok(val) => val,
                Err(e) => return e,
            };
            let user = database::User::Model::new(username, email, password);
            match insert_entity::<database::user::ActiveModel>(
                &db_connection,
                user.into_active_model(),
            )
            .await
            {
                Ok(user) => match serde_json::to_string(&user) {
                    Ok(json) => res_good(&json),
                    Err(_) => res_bad("Unable to convert user from database to Json"),
                },
                Err(e) => res_db_fail(e),
            }
        }
        "login" | "delete" | "update" => match get_user(&db_connection, &username).await {
            Ok(user_option) => match user_option {
                Some(user) => {
                    if user.password != password {
                        return res_unauth(
                            "Can't perform action due to user being not authenticated",
                        );
                    }
                    match action.as_str() {
                        "login" => match serde_json::to_string(&user) {
                            Ok(json) => res_good(&json),
                            Err(_) => res_bad("Unable to convert user from database to Json"),
                        },
                        "delete" => {
                            match delete_enitity(&db_connection, user.into_active_model()).await {
                                Ok(_) => res_good("User successfully deleted"),
                                Err(e) => res_db_fail(e),
                            }
                        }
                        "update" => {
                            let changed_password = match header_extract("changed_password", &header)
                            {
                                Ok(val) => val,
                                Err(e) => return e,
                            };
                            let changed_email = match header_extract("changed_email", &header) {
                                Ok(val) => val,
                                Err(_) => user.email,
                            };
                            let active_user = database::User::ActiveModel {
                                id: Set(user.id),
                                password: Set(changed_password),
                                email: Set(changed_email),
                                username: Set(user.username),
                            };
                            match update_entity(&db_connection, active_user).await {
                                Ok(_) => res_good("User successfully changed"),
                                Err(e) => res_db_fail(e),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                None => res_bad(&format!("No user with username '{username}' was found")),
            },
            Err(e) => res_db_fail(e),
        },
        _ => res_bad("Invalid action for users -> Valid actions login,delete,update,register"),
    }
}
