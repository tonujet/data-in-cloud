use crate::web::controller::PaginationParams;
use crate::web::error::ApiResult;
use crate::web::openapi::{ApiResponses, ObjectIdPathParam};
use crate::web::state::{AppState, UserRepoInfoState};

use super::super::EntityApi;
use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{Json, Router};
use dto::user_repo_info_dto::UserRepoInfoDto;
use dto::DtoList;
use mongodb::bson::oid::ObjectId;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_user_repo_info, list_user_repo_info,
    ),
    components(
        schemas(
            UserRepoInfoDto, DtoList<UserRepoInfoDto>,
        )
    ),
    tags(
        (name = EntityApi::UserRepoInfos.to_str_tag())
    ),
)]
pub struct UserRepoInfoOpenApi;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/:info_id", get(get_user_repo_info))
        .route("/", get(list_user_repo_info))
        .with_state(state)
}

#[utoipa::path(
    get,
    path = "/{id}",
    params(ObjectIdPathParam),
    responses (ApiResponses<UserRepoInfoDto>),
    tag = EntityApi::UserRepoInfos.to_str_tag(),
)]
async fn get_user_repo_info(
    State(state): State<UserRepoInfoState>,
    Path(info_id): Path<ObjectId>,
) -> ApiResult<Json<UserRepoInfoDto>> {
    let res = state.service.get(&info_id).await?;
    Ok(Json(res))
}

#[utoipa::path(
    get,
    path = "",
    params(PaginationParams),
    responses (ApiResponses<DtoList<UserRepoInfoDto>>),
    tag = EntityApi::UserRepoInfos.to_str_tag(),
)]
async fn list_user_repo_info(
    State(state): State<UserRepoInfoState>,
    Query(PaginationParams { take, offset }): Query<PaginationParams>,
) -> ApiResult<Json<DtoList<UserRepoInfoDto>>> {
    let res = state.service.list(take, offset).await?;
    Ok(Json(res))
}
