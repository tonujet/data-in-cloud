use axum::Router;

use crate::config::config;
use crate::error::InternalResult;
use crate::web::state::AppState;

mod controller;
mod error;
mod service;
pub mod state;
pub mod utils;


pub async fn start_server(sql_conn: sea_orm::DbConn, nosql_conn: mongodb::Database) -> InternalResult<()> {
    let listener = tokio::net::TcpListener::bind(&config().SERVER.SOCKET_ADDR).await?;
    println!("Server started on socket: {}", listener.local_addr()?);

    let state = state::create_state(sql_conn, nosql_conn).await?;
    let app = app(state);
    axum::serve(listener, app).await?;

    Ok(())
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .merge(controller::api_routes(state))
        .fallback(utils::api_not_found_handler)
}
