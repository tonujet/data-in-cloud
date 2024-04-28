use axum::Router;
use serde::Deserialize;

use super::error::ApiResult;
use super::state::*;

mod repo_controller;
mod user_controller;
mod user_repo_controller;

pub fn api_routes(state: AppState) -> Router {
    let router: Router<()> = Router::new();
    let api_router = Router::new()
        .nest("/repos", repo_controller::routes(state.clone()))
        .nest("/users", user_controller::routes(state.clone()))
        .nest("/users", user_repo_controller::routes(state.clone()));
    router.nest("/api/v1", api_router)
}

#[derive(Deserialize)]
struct PaginationParams {
    take: Option<u64>,
    offset: Option<u64>,
}
