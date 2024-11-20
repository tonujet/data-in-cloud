use crate::config::config;
use crate::error::InternalResult;
use crate::web::state::AppState;
use axum::Router;

pub mod api;
mod controller;
pub mod error;
mod openapi;
pub mod service;
pub mod state;
pub mod utils;

pub async fn start_server(state: AppState) -> InternalResult<()> {
    let listener = tokio::net::TcpListener::bind(&config().SERVER.SOCKET_ADDR).await?;
    println!("Server started on socket: {}", listener.local_addr()?);
    let app = app(state.clone());
    axum::serve(listener, app).await?;
    Ok(())
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .merge(controller::api_routes(state))
        .fallback(utils::api_not_found_handler)
}
