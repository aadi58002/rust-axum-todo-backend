mod connections;
mod entity;
mod actions;

use actions::table_actions::creation::{create_table, create_type};
pub use connections::*;
pub use entity::*;
pub use actions::*;

pub async fn init_tables(db_connection: &common::sea_orm::DatabaseConnection) -> Result<(),common::sea_orm::DbErr>{
    let _ = create_table(db_connection, User::Entity).await?;
    let _ = create_type::<Tasks::TaskState>(db_connection).await?;
    let _ = create_table(db_connection, Tasks::Entity).await?;
    Ok(())
}
