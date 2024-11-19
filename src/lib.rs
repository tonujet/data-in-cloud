mod config;
pub mod error;
pub mod web;
pub mod db;

pub async fn main() -> error::InternalResult<()>{
    let sql_conn = db::init_sql_database().await?;
    let nosql_conn = db::init_nosql_database().await?;
    web::start_server(sql_conn, nosql_conn).await?;
    Ok(())
}