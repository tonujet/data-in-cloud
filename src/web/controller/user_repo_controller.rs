use crate::web::controller::PaginationParams;
use crate::web::dto::user_repo_dto::{OneToManyDto, OneToOneDto};
use crate::web::error::ApiResult;
use crate::web::state::{AppState, UserRepoState};
use crate::web::openapi::{ApiResponses, ObjectIdPathParam, OpenApiDtoList};

use super::super::EntityApi;
use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use mongodb::bson::oid::ObjectId;
use utoipa::OpenApi;
use uuid::Uuid;
use repo::dto::repo_dto::RepoDto;
use repo::dto::user_dto::UserDto;

#[derive(OpenApi)]
#[openapi(
    paths(
        // add_pair, delete_pair, list_pairs,
    ),
    components(
        schemas()
    ),
    tags(
        (name = EntityApi::Users.to_tag())
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

async fn add_pair(
    State(state): State<UserRepoState>,
    Path((user_id, repo_id)): Path<(ObjectId, Uuid)>,
) -> ApiResult<Json<OneToOneDto<UserDto, RepoDto>>> {
    let res = state.service.add_pair(&user_id, &repo_id).await?;
    Ok(Json(res))
}

async fn delete_pair(
    State(state): State<UserRepoState>,
    Path((user_id, repo_id)): Path<(ObjectId, Uuid)>,
) -> ApiResult<Json<OneToOneDto<UserDto, RepoDto>>> {
    let res = state.service.delete_pair(&user_id, &repo_id).await?;
    Ok(Json(res))
}

// #[utoipa::path(
//     get,
//     path = "",
//     params(PaginationParams),
//     responses (ApiResponses<OpenApiDtoList<UserMultipleRepo>>),
//     tag = EntityApi::Users.to_tag(),
// )]
async fn list_pairs(
    State(state): State<UserRepoState>,
    Path(user_id): Path<ObjectId>,
    Query(PaginationParams { take, offset }): Query<PaginationParams>,
) -> ApiResult<Json<OneToManyDto<UserDto, RepoDto>>> {
    let res = state.service.list_pairs(&user_id, take, offset).await?;
    Ok(Json(res))
}
