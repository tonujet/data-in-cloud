use axum::Router;
use sea_orm::DbConn;

use crate::config::config;
use crate::error::InternalResult;
use crate::web::state::AppState;

mod controller;
pub mod dto;
mod error;
mod service;
pub mod state;
pub mod utils;


pub async fn start_server(conn: DbConn) -> InternalResult<()> {
    let listener = tokio::net::TcpListener::bind(&config().SERVER.SOCKET_ADDR).await?;
    println!("Server started on socket: {}", listener.local_addr()?);

    let state = state::create_state(conn);
    let app = app(state);
    axum::serve(listener, app).await?;

    Ok(())
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .merge(controller::api_routes(state))
        .fallback(utils::api_not_found_handler)
}
