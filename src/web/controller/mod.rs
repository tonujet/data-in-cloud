use axum::Router;
use repo::graphql_repo_controller::{MutationRepo, QueryRepo};
use repo::rest_repo_controller;
use serde::Deserialize;
use user::graphql_user_controller::{MutationUser, QueryUser};
use user::rest_user_controller;
use user_repo_info::graphql_user_repo_info_controller::QueryUserRepoInfo;
use user_repo_info::rest_user_repo_info_controller;

mod repo;
mod user;
mod user_repo_controller;
mod user_repo_info;

use crate::web::state::AppState;
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema, SimpleObject};
use async_graphql_axum::GraphQL;
use axum::response::{self, IntoResponse};
use axum::routing::get;

const API_PREFIX: &str = "/api/v1";

pub async fn graphiql() -> impl IntoResponse {
    let endpoint = format!("{API_PREFIX}/graphql");
    response::Html(GraphiQLSource::build().endpoint(&endpoint).finish())
}

#[derive(SimpleObject, Default)]
pub struct MutationRoot {
    repos: MutationRepo,
    users: MutationUser,
}

#[derive(SimpleObject, Default)]
pub struct QueryRoot {
    repos: QueryRepo,
    users: QueryUser,
    user_repo_infos: QueryUserRepoInfo,
}

fn schema(state: AppState) -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(state)
    .finish()
}

pub fn api_routes(state: AppState) -> Router {
    let router: Router<()> = Router::new();
    let schema = schema(state.clone());
    let api_router = Router::new()
        .route("/graphql", get(graphiql).post_service(GraphQL::new(schema)))
        .nest("/repos", rest_repo_controller::routes(state.clone()))
        .nest("/users", rest_user_controller::routes(state.clone()))
        .nest("/users", user_repo_controller::routes(state.clone()))
        .nest(
            "/user-repo-infos",
            rest_user_repo_info_controller::routes(state.clone()),
        );
    router.nest(API_PREFIX, api_router)
}

#[derive(Deserialize)]
struct PaginationParams {
    take: Option<u64>,
    offset: Option<u64>,
}
