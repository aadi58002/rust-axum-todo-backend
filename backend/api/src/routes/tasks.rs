use core::entity_actions::deletion::delete_enitity;
use core::entity_actions::get::{get_all_tasks, get_user, get_task};
use core::entity_actions::save::save_entity;
use common::sea_orm::IntoActiveModel;
use core::Tasks;

use common::axum::http::HeaderMap;
use common::axum::response::IntoResponse;
use common::axum::{Extension, Json};

use crate::helper::header_extract::*;
use crate::helper::res_con::{res_bad, res_good, res_db_fail};

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

    let user = match get_user(&db_connection, &username).await{
        Ok(maybe_user) => match maybe_user {
            Some(user) => user,
            None => return res_bad(&format!("No user with the username {}",username)),
        },
        Err(e) => return res_db_fail(e),
    };

    let title = match header_extract("title", &header) {
        Ok(val) => val,
        Err(e) => return e,
    };
    match action.as_str() {
        "get" => {
            match get_all_tasks(&db_connection, user).await{
                Ok(tasks) => {
                },
                Err(e) => return res_db_fail(e),
            }
        },
        "add" => {
            let description = match header_extract("description", &header) {
                Ok(val) => val,
                Err(e) => return e,
            };

            let new_task = core::Tasks::Model::new(username, title,description);
            match save_entity::<core::Tasks::ActiveModel>(
                &db_connection,
                new_task.into_active_model(),
            )
            .await
            {
                Ok(_) => res_good("New task created"),
                Err(e) => res_bad(&format!("Unable to create the task : {}", e)),
            }
        }
        "delete" => {
            match get_task(&db_connection, user,title).await{
                Ok(maybe_task) => match maybe_task {
                    Some(task) => match delete_enitity::<Tasks::ActiveModel>(&db_connection, task.into_active_model()).await{
                        Ok(_) => return res_good("Task successful deleted"),
                        Err(e) => return res_db_fail(e),
                    },
                    None => return res_bad("There is no task with the provided title"),
                },
                Err(e) => return res_db_fail(e),

            }
        }
        _ => res_bad("Invalid Command detected -> Valid command add"),
    }
}
