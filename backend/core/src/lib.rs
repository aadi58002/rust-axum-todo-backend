mod connections;
mod entity;
mod actions;

use actions::table_actions::creation::{create_table, create_type};
pub use connections::connections::*;
pub use entity::*;
pub use actions::*;
pub use sea_orm;
use sea_orm::{DatabaseConnection, DbErr};

pub async fn init_tables(db_connection: &DatabaseConnection) -> Result<(),DbErr>{
    let _ = create_table(db_connection, User::Entity).await?;
    let _ = create_type::<Tasks::TaskState>(db_connection).await?;
    let _ = create_table(db_connection, Tasks::Entity).await?;
    Ok(())
}
