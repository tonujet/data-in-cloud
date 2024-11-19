use axum::{Json, Router};
use axum::extract::{Path, Query, State};
use axum::routing::get;
use mongodb::bson::oid::ObjectId;
use repo::dto::DtoList;
use repo::dto::user_repo_info_dto::UserRepoInfoDto;

use crate::web::controller::PaginationParams;
use crate::web::error::ApiResult;
use crate::web::state::{AppState, UserRepoInfoState};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/:info_id", get(get_user_repo_info))
        .route("/", get(list_user_repo_info))
        .with_state(state)
}

async fn get_user_repo_info(
    State(state): State<UserRepoInfoState>,
    Path(info_id): Path<ObjectId>,
) -> ApiResult<Json<UserRepoInfoDto>> {
    let res = state.service.get(&info_id).await?;
    Ok(Json(res))
}

async fn list_user_repo_info(
    State(state): State<UserRepoInfoState>,
    Query(PaginationParams { take, offset }): Query<PaginationParams>,
) -> ApiResult<Json<DtoList<UserRepoInfoDto>>> {
    let res = state.service.list(take, offset).await?;
    Ok(Json(res))
}
