use core::{
    init_tables,
    table_actions::{clear::clear_table, drop::drop_table},
};

use common::axum::{
    http::HeaderMap,
    response::IntoResponse,
    Extension,
};

use crate::helper::{header_extract::header_extract, res_con::*};

pub async fn table(
    Extension(db_connection): Extension<common::sea_orm::DatabaseConnection>,
    header: HeaderMap,
) -> impl IntoResponse {
    let action = match header_extract("action", &header) {
        Ok(val) => val,
        Err(e) => return e,
    };
    match action.as_str() {
        "clear" => {
            clear_table(&db_connection, core::User::Entity);
            clear_table(&db_connection, core::Tasks::Entity);
            res_good("Tables Cleared")
        }
        "init" => match init_tables(&db_connection).await {
            Ok(_) => res_good("Tables Created"),
            Err(e) => res_db_fail(e.to_string()),
        },
        "drop" => {
            drop_table(&db_connection, core::User::Entity);
            drop_table(&db_connection, core::Tasks::Entity);
            res_good("Tables Droped")
        }
        _ => res_bad(&format!("Instruction is invalid {}", action))
    }
}
