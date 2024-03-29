mod actions;
mod connections;
mod entity;

use actions::table_actions::creation::{create_table, create_type};
pub use actions::*;
pub use connections::*;
pub use entity::*;
pub use sea_orm::DatabaseConnection;

pub async fn init_tables(
    db_connection: &sea_orm::DatabaseConnection,
) -> Result<(), sea_orm::DbErr> {
    let _ = create_table(db_connection, User::Entity).await?;
    let _ = create_type::<Task::TaskState>(db_connection).await?;
    let _ = create_table(db_connection, Task::Entity).await?;
    Ok(())
}
