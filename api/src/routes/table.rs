use database::{
    init_tables,
    table_actions::{clear::clear_table, drop::{drop_table, drop_type}},
};

use axum::{
    http::HeaderMap,
    response::IntoResponse,
    Extension,
};

use crate::helper::{header_extract::header_extract, res_con::*};

pub async fn table(
    Extension(db_connection): Extension<sea_orm::DatabaseConnection>,

    header: HeaderMap,
) -> impl IntoResponse {
    let action = match header_extract("action", &header) {
        Ok(val) => val,
        Err(e) => return e,
    };
    match action.as_str() {
        "clear" => {
            if let Err(e) = clear_table(&db_connection, database::Task::Entity).await{
                return res_bad(&format!("Unable to clear table Tasks : {}",e));
            }
            if let Err(e) = clear_table(&db_connection, database::User::Entity).await{
                return res_bad(&format!("Unable to clear table User : {}",e));
            }
            res_good("Tables Cleared")
        }
        "init" => match init_tables(&db_connection).await {
            Ok(_) => res_good("Tables Created"),
            Err(e) => res_db_fail(e.to_string()),
        },
        "drop" => {
            if let Err(e) = drop_table(&db_connection, database::Task::Entity).await{
                return res_bad(&format!("Unable to drop table Tasks : {}",e));
            }
            if let Err(e) = drop_type::<database::Task::TaskState>(&db_connection).await{
                return res_bad(&format!("Unable to drop type TaskState : {}",e));
            }
            if let Err(e) = drop_table(&db_connection, database::User::Entity).await{
                return res_bad(&format!("Unable to drop table User : {}",e));
            }
            res_good("Tables Droped")
        }
        _ => res_bad(&format!("Instruction is invalid {}", action))
    }
}
