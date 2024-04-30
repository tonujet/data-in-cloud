use mongodb::bson::doc;
use migration::{Migrator, MigratorTrait};

use crate::config::config;
use crate::error::InternalResult;

pub async fn init_sql_database() -> InternalResult<sea_orm::DbConn> {
    let conn = sea_orm::Database::connect(&config().SQL_DB.URL).await?;
    println!("Connected to sql database");
    Migrator::up(&conn, None).await?;
    println!("Migrations to sql database is executed");
    Ok(conn)
}

pub async fn init_nosql_database() -> InternalResult<mongodb::Database> {
    let mut client_options = mongodb::options::ClientOptions::parse(&config().MONGO_DB.URL).await?;
    client_options.app_name = Some(config().MONGO_DB.NAME.to_string());
    let client = mongodb::Client::with_options(client_options)?;
    let conn = client.database(&config().MONGO_DB.NAME);
    let cmd = doc! { "ping": 1 };
    conn.run_command(cmd, None).await?;
    println!("Connected to mongodb database {}", &config().MONGO_DB.NAME);
    Ok(conn)
}


pub async fn init_test_sql_database() -> sea_orm::DbConn {
    let conn = sea_orm::Database::connect(&config().SQL_DB.TEST_URL).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();
    conn
}
