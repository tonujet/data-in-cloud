use axum::extract::{Path, Query, State};
use axum::routing::{get, post, put};
use axum::{Json, Router};
use mongodb::bson::oid::ObjectId;

use crate::web::controller::PaginationParams;
use crate::web::error::ApiResult;
use crate::web::state::{AppState, UserState};
use crate::web::utils::validation::ValidationWrapper;
use repo::dto::user_dto::{CreateUserDto, UpdateUserDto, UserDto};
use repo::dto::user_repo_info_dto::UserRepoInfoDto;
use repo::dto::DtoList;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", post(create_user).get(list_users))
        .route("/:id", put(update_user).get(get_user).delete(delete_user))
        .route("/:id/repo-infos", get(list_user_repos_info))
        .with_state(state)
}

async fn create_user(
    State(state): State<UserState>,
    user_dto: ValidationWrapper<CreateUserDto>,
) -> ApiResult<Json<UserDto>> {
    let user = state.service.create(user_dto.0).await?;
    Ok(Json(user))
}

async fn update_user(
    State(state): State<UserState>,
    Path(id): Path<ObjectId>,
    user_dto: ValidationWrapper<UpdateUserDto>,
) -> ApiResult<Json<UserDto>> {
    let user = state.service.update(&id, user_dto.0).await?;
    Ok(Json(user))
}

async fn list_users(
    State(state): State<UserState>,
    Query(PaginationParams { take, offset }): Query<PaginationParams>,
) -> ApiResult<Json<DtoList<UserDto>>> {
    let users = state.service.list(take, offset).await?;
    Ok(Json(users))
}

async fn list_user_repos_info(
    State(state): State<UserState>,
    Path(id): Path<ObjectId>,
    Query(PaginationParams { take, offset }): Query<PaginationParams>,
) -> ApiResult<Json<DtoList<UserRepoInfoDto>>> {
    let users = state.service.list_user_repos_info(id, take, offset).await?;
    Ok(Json(users))
}

async fn get_user(
    State(state): State<UserState>,
    Path(id): Path<ObjectId>,
) -> ApiResult<Json<UserDto>> {
    let user = state.service.get(&id).await?;
    Ok(Json(user))
}

async fn delete_user(
    State(state): State<UserState>,
    Path(id): Path<ObjectId>,
) -> ApiResult<Json<UserDto>> {
    let user = state.service.delete(&id).await?;
    Ok(Json(user))
}