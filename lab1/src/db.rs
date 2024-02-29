use sea_orm::{Database, DbConn};

use migration::{Migrator, MigratorTrait};

use crate::config::config;
use crate::error::InternalResult;

pub async fn init_database() -> InternalResult<DbConn> {
    let conn = Database::connect(&config().DB.URL).await?;
    println!("Connected to database");
    Migrator::up(&conn, None).await?;
    println!("Migrations executed");
    Ok(conn)
}

pub async fn init_test_database() -> DbConn {
    let conn = Database::connect(&config().DB.TEST_URL).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();
    conn
}
