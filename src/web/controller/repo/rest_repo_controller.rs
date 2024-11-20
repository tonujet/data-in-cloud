use axum::extract::{Path, Query, State};
use axum::routing::{post, put};
use axum::{Json, Router};
use utoipa::OpenApi;
use uuid::Uuid;

use super::super::EntityApi;
use crate::web::controller::PaginationParams;
use crate::web::error::ApiResult;
use crate::web::openapi::{ApiResponses, UuidPathParam};
use crate::web::state::{AppState, RepoState};
use crate::web::utils::validation::ValidationWrapper;
use entity::RepositoryType;
use dto::repo_dto::CreateUpdateRepoDto;
use dto::{repo_dto::RepoDto, DtoList};

#[derive(OpenApi)]
#[openapi(
    paths(
        get_repo, delete_repo,
        list_repos, create_repo,
        update_repo,
    ),
    components(
        schemas(
            RepoDto, RepositoryType,
            CreateUpdateRepoDto, DtoList<RepoDto>
        )
    ),
    tags(
        (name = EntityApi::Repos.to_str_tag())
    ),
)]
pub struct RepoOpenApi;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", post(create_repo).get(list_repos))
        .route("/:id", put(update_repo).get(get_repo).delete(delete_repo))
        .with_state(state)
}

#[utoipa::path(
    post,
    path = "",
    request_body = CreateUpdateRepoDto,
    responses (ApiResponses<RepoDto>),
    tag = EntityApi::Repos.to_str_tag(),
)]
async fn create_repo(
    State(state): State<RepoState>,
    repo_dto: ValidationWrapper<CreateUpdateRepoDto>,
) -> ApiResult<Json<RepoDto>> {
    let repo = state.service.create(repo_dto.0).await?;
    Ok(Json(repo))
}

#[utoipa::path(
    put,
    path = "/{id}",
    params(UuidPathParam),
    request_body = CreateUpdateRepoDto,
    responses (ApiResponses<RepoDto>),
    tag = EntityApi::Repos.to_str_tag(),
)]
async fn update_repo(
    State(state): State<RepoState>,
    Path(id): Path<Uuid>,
    repo_dto: ValidationWrapper<CreateUpdateRepoDto>,
) -> ApiResult<Json<RepoDto>> {
    let repo = state.service.update(&id, repo_dto.0).await?;
    Ok(Json(repo))
}

#[utoipa::path(
    get,
    path = "",
    params(PaginationParams),
    responses (ApiResponses<DtoList<RepoDto>>),
    tag = EntityApi::Repos.to_str_tag(),
)]
async fn list_repos(
    State(state): State<RepoState>,
    Query(PaginationParams { take, offset }): Query<PaginationParams>,
) -> ApiResult<Json<DtoList<RepoDto>>> {
    let repos = state.service.list(take, offset).await?;
    Ok(Json(repos))
}

#[utoipa::path(
    get,
    path = "/{id}",
    params(UuidPathParam),
    responses (ApiResponses<RepoDto>),
    tag = EntityApi::Repos.to_str_tag(),
)]
async fn get_repo(
    State(state): State<RepoState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<RepoDto>> {
    let repo = state.service.get(&id).await?;
    Ok(Json(repo))
}

#[utoipa::path(
    delete,
    path = "/{id}",
    params(UuidPathParam),
    responses (ApiResponses<RepoDto>),
    tag = EntityApi::Repos.to_str_tag(),
)]
async fn delete_repo(
    State(state): State<RepoState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<RepoDto>> {
    let repo = state.service.delete(&id).await?;
    Ok(Json(repo))
}
