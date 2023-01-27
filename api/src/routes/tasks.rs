use crate::helper::{
    header_extract::*,
    res_con::{res_bad, res_db_fail, res_good},
};
use common::axum::{http::HeaderMap, response::IntoResponse, Extension};
use common::sea_orm::{entity::*, IntoActiveModel};
use core::entity_actions::{
    deletion::delete_enitity, get::*, insert::insert_entity, update::update_entity,
};

pub async fn tasks(
    Extension(db_connection): Extension<common::sea_orm::DatabaseConnection>,
    header: HeaderMap,
) -> impl IntoResponse {
    let action = match header_extract("action", &header) {
        Ok(val) => val,
        Err(e) => return e,
    };
    let username = match header_extract("username", &header) {
        Ok(val) => val,
        Err(e) => return e,
    };

    let password = match header_extract("password", &header) {
        Ok(val) => val,
        Err(e) => return e,
    };

    let user = match get_user(&db_connection, &username).await {
        Ok(maybe_user) => match maybe_user {
            Some(user) => user,
            None => return res_bad(&format!("No user with the username {}", username)),
        },
        Err(e) => return res_db_fail(e),
    };

    if user.password != password {
        return res_bad("Can't perform action due to user being not authenticated");
    }
    match action.as_str() {
        "get" => match get_all_tasks(&db_connection, user).await {
            Ok(tasks) => match serde_json::to_string(&tasks) {
                Ok(json) => res_good(&json),
                Err(_) => res_bad("Unable to convert the values coming from database to json"),
            },
            Err(e) => return res_db_fail(e),
        },
        "add" | "delete" | "update" => {
            let title = match header_extract("title", &header) {
                Ok(val) => val,
                Err(e) => return e,
            };
            match action.as_str().clone() {
                "add" => {
                    let description = match header_extract("description", &header) {
                        Ok(val) => Some(val),
                        Err(_) => None,
                    };

                    let new_task = core::Tasks::Model::new(username, title, description);
                    match insert_entity::<core::Tasks::ActiveModel>(
                        &db_connection,
                        new_task.into_active_model(),
                    )
                    .await
                    {
                        Ok(_) => res_good("New task created"),
                        Err(e) => res_bad(&format!("Unable to create the task : {}", e)),
                    }
                }
                "delete" => match get_task(&db_connection, user, title).await {
                    Ok(maybe_task) => match maybe_task {
                        Some(task) => match delete_enitity::<core::Tasks::ActiveModel>(
                            &db_connection,
                            task.into_active_model(),
                        )
                        .await
                        {
                            Ok(_) => return res_good("Task successful deleted"),
                            Err(e) => return res_db_fail(e),
                        },
                        None => return res_bad("There is no task with the provided title"),
                    },
                    Err(e) => return res_db_fail(e),
                },
                "update" => match get_task(&db_connection, user, title).await {
                    Ok(maybe_task) => match maybe_task {
                        Some(task) => {
                            let changed_title = match header_extract("changed_title", &header) {
                                Ok(val) => val,
                                Err(_) => task.title,
                            };

                            let changed_description =
                                match header_extract("changed_desription", &header) {
                                    Ok(val) => Some(val),
                                    Err(_) => None,
                                };

                            let changed_status = match header_extract("changed_status", &header) {
                                Ok(val) => match val.as_str() {
                                    "Pending" => core::Tasks::TaskState::Pending,
                                    "Completed" => core::Tasks::TaskState::Completed,
                                    _ => unreachable!(),
                                },
                                Err(_) => task.status,
                            };
                            let active_task = core::Tasks::ActiveModel {
                                id: Set(task.id),
                                username: Set(task.username),
                                title: Set(changed_title),
                                description: Set(changed_description),
                                status: Set(changed_status),
                            };
                            match update_entity::<core::Tasks::ActiveModel>(
                                &db_connection,
                                active_task,
                            )
                            .await
                            {
                                Ok(_) => return res_good("Task successful updated"),
                                Err(e) => return res_db_fail(e),
                            }
                        }
                        None => return res_bad("There is no task with the provided title"),
                    },
                    Err(e) => return res_db_fail(e),
                },
                _ => unreachable!(),
            }
        }
        _ => res_bad("Invalid Command detected -> Valid actions add,delete,update,get"),
    }
}
