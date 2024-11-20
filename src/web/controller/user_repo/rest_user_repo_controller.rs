use crate::web::controller::PaginationParams;
use crate::web::error::ApiResult;
use crate::web::openapi::{ApiResponses, ObjectIdPathParam};
use crate::web::state::{AppState, UserRepoState};

use super::super::EntityApi;
use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use dto::repo_dto::RepoDto;
use dto::user_dto::UserDto;
use dto::{OneToManyDto, OneToOneDto};
use mongodb::bson::oid::ObjectId;
use utoipa::OpenApi;
use uuid::Uuid;

#[derive(OpenApi)]
#[openapi(
    paths(
        list_pairs, delete_pair, add_pair,
    ),
    components(
        schemas(
            OneToManyDto<UserDto, RepoDto>,
            OneToOneDto<UserDto, RepoDto>,
        )
    ),
    tags(
        (name = EntityApi::Users.to_str_tag())
    ),
)]
pub struct UserRepoOpenApi;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route(
            "/:user_id/repos/:repo_id",
            post(add_pair).delete(delete_pair),
        )
        .route("/:user_id/repos", get(list_pairs))
        .with_state(state)
}

#[utoipa::path(
    post,
    path = "/{user_id}/repos/{repo_id}",
    params(
        ("user_id" = String, Path, pattern = "^[0-9a-fA-F]{24}$"),
        ("repo_id" = Uuid, Path),
    ),
    responses (ApiResponses<OneToOneDto<UserDto, RepoDto>>),
    tag = EntityApi::Users.to_str_tag(),
)]
async fn add_pair(
    State(state): State<UserRepoState>,
    Path((user_id, repo_id)): Path<(ObjectId, Uuid)>,
) -> ApiResult<Json<OneToOneDto<UserDto, RepoDto>>> {
    let res = state.service.add_pair(&user_id, &repo_id).await?;
    Ok(Json(res))
}

#[utoipa::path(
    delete,
    path = "/{user_id}/repos/{repo_id}",
    params(
        ("user_id" = String, Path, pattern = "^[0-9a-fA-F]{24}$"),
        ("repo_id" = Uuid, Path),
    ),
    responses (ApiResponses<OneToManyDto<UserDto, RepoDto>>),
    tag = EntityApi::Users.to_str_tag(),
)]
async fn delete_pair(
    State(state): State<UserRepoState>,
    Path((user_id, repo_id)): Path<(ObjectId, Uuid)>,
) -> ApiResult<Json<OneToOneDto<UserDto, RepoDto>>> {
    let res = state.service.delete_pair(&user_id, &repo_id).await?;
    Ok(Json(res))
}

#[utoipa::path(
    get,
    path = "/{id}/repos",
    params(
        ObjectIdPathParam,
        PaginationParams,
    ),
    responses (ApiResponses<OneToOneDto<UserDto, RepoDto>>),
    tag = EntityApi::Users.to_str_tag(),
)]
async fn list_pairs(
    State(state): State<UserRepoState>,
    Path(user_id): Path<ObjectId>,
    Query(PaginationParams { take, offset }): Query<PaginationParams>,
) -> ApiResult<Json<OneToManyDto<UserDto, RepoDto>>> {
    let res = state.service.list_pairs(&user_id, take, offset).await?;
    Ok(Json(res))
}
