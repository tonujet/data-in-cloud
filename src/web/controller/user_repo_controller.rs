use axum::extract::{Path, Query, State};
use axum::{Json, Router};
use axum::routing::{get, post};
use mongodb::bson::oid::ObjectId;
use uuid::Uuid;
use crate::web::controller::PaginationParams;
use crate::web::dto::user_repo_dto::{UserMultipleRepo, UserSingleRepo};
use crate::web::error::ApiResult;
use crate::web::state::{AppState, UserRepoState};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/:user_id/repos/:repo_id", post(add_pair).delete(delete_pair))
        .route("/:user_id/repos", get(list_pairs))
        .with_state(state)
}


async fn add_pair(
    State(state): State<UserRepoState>,
    Path((user_id, repo_id)): Path<(ObjectId, Uuid)>,
) -> ApiResult<Json<UserSingleRepo>> {
    let res = state.service.add_pair(&user_id, &repo_id).await?;
    Ok(Json(res))
}

async fn delete_pair(
    State(state): State<UserRepoState>,
    Path((user_id, repo_id)): Path<(ObjectId, Uuid)>,
) -> ApiResult<Json<UserSingleRepo>>{
    let res = state.service.delete_pair(&user_id, &repo_id).await?;
    Ok(Json(res))
}

async fn list_pairs(
    State(state): State<UserRepoState>,
    Path(user_id): Path<ObjectId>,
    Query(PaginationParams { take, offset }): Query<PaginationParams>,
) -> ApiResult<Json<UserMultipleRepo>>{
    let res = state.service.list_pairs(&user_id, take, offset).await?;
    Ok(Json(res))
}