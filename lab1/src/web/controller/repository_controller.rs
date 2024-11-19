use axum::{Json, Router};
use axum::extract::{Path, Query, State};
use axum::routing::{post, put};
use uuid::Uuid;

use super::{ApiResult, PaginationParams};
use super::{AppState, RepoState};
use repo::dto::{ListResponse, repository_dto::ResponseRepoDto};
use crate::web::dto::CreateUpdateRepoDtoWrapper;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", post(create_repo).get(list_repos))
        .route("/:id", put(update_repo).get(get_repo).delete(delete_repo))
        .with_state(state)
}

async fn create_repo(
    State(state): State<RepoState>,
    repo_dto: CreateUpdateRepoDtoWrapper,
) -> ApiResult<Json<ResponseRepoDto>> {
    let repo = state.service.create(repo_dto.0).await?;
    Ok(Json(repo))
}

async fn update_repo(
    State(state): State<RepoState>,
    Path(id): Path<Uuid>,
    repo_dto: CreateUpdateRepoDtoWrapper,
) -> ApiResult<Json<ResponseRepoDto>> {
    let repo = state.service.update(id, repo_dto.0).await?;
    Ok(Json(repo))
}

async fn list_repos(
    State(state): State<RepoState>,
    Query(PaginationParams { take, offset }): Query<PaginationParams>,
) -> ApiResult<Json<ListResponse<ResponseRepoDto>>> {
    let repos = state.service.list(take, offset).await?;
    Ok(Json(repos))
}

async fn get_repo(
    State(state): State<RepoState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ResponseRepoDto>> {
    let repo = state.service.get(id).await?;
    Ok(Json(repo))
}

async fn delete_repo(
    State(state): State<RepoState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ResponseRepoDto>> {
    let repo = state.service.delete(id).await?;
    Ok(Json(repo))
}
