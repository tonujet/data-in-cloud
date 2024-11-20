use crate::web::controller::PaginationParams;
use crate::web::error::ApiResult;
use crate::web::openapi::{ApiResponses, ObjectIdPathParam};
use crate::web::state::{AppState, UserState};
use crate::web::utils::validation::ValidationWrapper;
use axum::extract::{Path, Query, State};
use axum::routing::{get, post, put};
use axum::{Json, Router};
use mongodb::bson::oid::ObjectId;
use utoipa::OpenApi;

use super::super::EntityApi;
use dto::user_dto::{CreateUserDto, UpdateUserDto, UserDto};
use dto::user_repo_info_dto::UserRepoInfoDto;
use dto::DtoList;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_user, delete_user,
        list_users, create_user,
        update_user, list_user_repos_info,
    ),
    components(
        schemas(
            UserDto, CreateUserDto,
            UpdateUserDto, DtoList<UserDto>,
            DtoList<UserRepoInfoDto>,
        )
    ),
    tags(
        (name = EntityApi::Users.to_str_tag())
    ),
)]
pub struct UserOpenApi;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", post(create_user).get(list_users))
        .route("/:id", put(update_user).get(get_user).delete(delete_user))
        .route("/:id/repo-infos", get(list_user_repos_info))
        .with_state(state)
}

#[utoipa::path(
    post,
    path = "",
    request_body = CreateUserDto,
    responses (ApiResponses<UserDto>),
    tag = EntityApi::Users.to_str_tag(),
)]
async fn create_user(
    State(state): State<UserState>,
    user_dto: ValidationWrapper<CreateUserDto>,
) -> ApiResult<Json<UserDto>> {
    let user = state.service.create(user_dto.0).await?;
    Ok(Json(user))
}

#[utoipa::path(
    put,
    path = "/{id}",
    params(ObjectIdPathParam),
    request_body = UpdateUserDto,
    responses (ApiResponses<UserDto>),
    tag = EntityApi::Users.to_str_tag(),
)]
async fn update_user(
    State(state): State<UserState>,
    Path(id): Path<ObjectId>,
    user_dto: ValidationWrapper<UpdateUserDto>,
) -> ApiResult<Json<UserDto>> {
    let user = state.service.update(&id, user_dto.0).await?;
    Ok(Json(user))
}

#[utoipa::path(
    get,
    path = "",
    params(PaginationParams),
    responses (ApiResponses<DtoList<UserDto>>),
    tag = EntityApi::Users.to_str_tag(),
)]
async fn list_users(
    State(state): State<UserState>,
    Query(PaginationParams { take, offset }): Query<PaginationParams>,
) -> ApiResult<Json<DtoList<UserDto>>> {
    let users = state.service.list(take, offset).await?;
    Ok(Json(users))
}

#[utoipa::path(
    get,
    path = "/{id}/repo-infos",
    params(ObjectIdPathParam, PaginationParams),
    responses (ApiResponses<DtoList<UserRepoInfoDto>>),
    tag = EntityApi::Users.to_str_tag(),
)]
async fn list_user_repos_info(
    State(state): State<UserState>,
    Path(id): Path<ObjectId>,
    Query(PaginationParams { take, offset }): Query<PaginationParams>,
) -> ApiResult<Json<DtoList<UserRepoInfoDto>>> {
    let users = state.service.list_user_repos_info(id, take, offset).await?;
    Ok(Json(users))
}

#[utoipa::path(
    get,
    path = "/{id}",
    params(ObjectIdPathParam),
    responses (ApiResponses<UserDto>),
    tag = EntityApi::Users.to_str_tag(),
)]
async fn get_user(
    State(state): State<UserState>,
    Path(id): Path<ObjectId>,
) -> ApiResult<Json<UserDto>> {
    let user = state.service.get(&id).await?;
    Ok(Json(user))
}

#[utoipa::path(
    delete,
    path = "/{id}",
    params(ObjectIdPathParam),
    responses (ApiResponses<UserDto>),
    tag = EntityApi::Users.to_str_tag(),
)]
async fn delete_user(
    State(state): State<UserState>,
    Path(id): Path<ObjectId>,
) -> ApiResult<Json<UserDto>> {
    let user = state.service.delete(&id).await?;
    Ok(Json(user))
}
