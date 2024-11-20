use axum::Router;
use repo::graphql_repo_controller::{MutationRepo, QueryRepo};
use repo::rest_repo_controller::{self, RepoOpenApi};
use serde::Deserialize;
use user::graphql_user_controller::{MutationUser, QueryUser};
use user::rest_user_controller::{self, UserOpenApi};
use user_repo_info::graphql_user_repo_info_controller::QueryUserRepoInfo;
use user_repo_info::rest_user_repo_info_controller::{self, UserRepoInfoOpenApi};
use user_repo::rest_user_repo_controller::{self, UserRepoOpenApi};


use crate::web::error::ApiErrorResponse;


mod repo;
mod user;
mod user_repo_info;
mod user_repo;

use crate::web::state::AppState;
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema, SimpleObject};
use async_graphql_axum::GraphQL;
use axum::response::{self, IntoResponse};
use axum::routing::get;
use utoipa::openapi::{Info, OpenApiBuilder};
use once_cell::sync::Lazy;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::web::EntityApi;

const API_VERSION: u8 = 1;

static API_PREFIX: Lazy<String> = Lazy::new(|| format!("/api/v{}", API_VERSION));


pub async fn graphiql() -> impl IntoResponse {
    let endpoint = format!("{}/graphql", *API_PREFIX);
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

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = EntityApi::Repos.to_endpoint(), api = RepoOpenApi),
        (path = EntityApi::Users.to_endpoint(), api = UserOpenApi),
        (path = EntityApi::Users.to_endpoint(), api = UserRepoOpenApi),
        (path = EntityApi::UserRepoInfos.to_endpoint(), api = UserRepoInfoOpenApi),
    ),
    components(
        schemas(ApiErrorResponse<String>)
    )
)]
struct ApiDoc;



pub fn api_routes(state: AppState) -> Router {
    let schema = schema(state.clone());
    let api_router = Router::new()
        .route("/graphql", get(graphiql).post_service(GraphQL::new(schema)))
        .nest(EntityApi::Repos.to_endpoint(), rest_repo_controller::routes(state.clone()))
        .nest(EntityApi::Users.to_endpoint(), rest_user_controller::routes(state.clone()))
        .nest(EntityApi::Users.to_endpoint(), rest_user_repo_controller::routes(state.clone()))
        .nest(
            EntityApi::UserRepoInfos.to_endpoint(),
            rest_user_repo_info_controller::routes(state.clone()),
        );


    let api_version_doc = OpenApiBuilder::new()
        .info(Info::new("Data in cloud API", API_VERSION.to_string().as_str()))
        .build()
        .nest(API_PREFIX.as_str(), ApiDoc::openapi());
    let swagger_router = SwaggerUi::new("/swagger-ui")
        .url(format!("{}/openapi.json", *API_PREFIX), api_version_doc);


    Router::new()
        .nest(API_PREFIX.as_str(), api_router)
        .merge(swagger_router)
}

#[derive(Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct PaginationParams {
    /// How many items to take
    // #[param(required = false)]
    take: Option<u64>,

    /// Offset before taking items
    // #[param(required = false)]
    offset: Option<u64>,
}
