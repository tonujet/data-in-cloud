mod config;
pub mod error;
pub mod web;
pub mod db;

pub async fn main() -> error::InternalResult<()>{
    let conn = db::init_database().await?;
    web::start_server(conn).await?;
    Ok(())
}