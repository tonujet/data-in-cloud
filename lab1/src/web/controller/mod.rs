use axum::Router;
use serde::{Deserialize};

use super::error::ApiResult;
use super::state::*;

mod repository_controller;

pub fn api_routes(state: AppState) -> Router {
    let router: Router<()> = Router::new();
    let api_router = Router::new().nest("/repo", repository_controller::routes(state));
    router.nest("/apiV1", api_router)
}

#[derive(Deserialize)]
struct PaginationParams {
    take: Option<u64>,
    offset: Option<u64>,
}
