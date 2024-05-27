use crate::web::state::AppState;

mod runtime;
mod config;
pub mod db;
pub mod error;
pub mod web;
mod message_broker;

pub async fn main() -> error::InternalResult<()> {
    let sql_conn = db::init_sql_database().await?;
    let nosql_conn = db::init_nosql_database().await?;
    let rabbitmq_conn = message_broker::get_rabbitmq_connection().await?;

    let state = AppState::build(sql_conn, nosql_conn, rabbitmq_conn).await?;
    runtime::run_detached_tasks(&state);
    web::start_server(state).await?;
    Ok(())
}
