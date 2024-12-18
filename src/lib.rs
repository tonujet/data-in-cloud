use crate::web::state::AppState;

mod config;
pub mod db;
pub mod error;
mod message_broker;
mod runtime;
pub mod web;

// TODO Think about the change of generics in general repository and service traits to the associated types
// TODO Think about urls in tests and version based technique
pub async fn main() -> error::InternalResult<()> {
    let sql_conn = db::init_sql_database().await?;
    let nosql_conn = db::init_nosql_database().await?;
    let rabbitmq_conn = message_broker::get_rabbitmq_connection().await?;

    let state = AppState::build(sql_conn, nosql_conn, rabbitmq_conn).await?;
    runtime::run_detached_tasks(&state);
    web::start_server(state).await?;
    Ok(())
}
