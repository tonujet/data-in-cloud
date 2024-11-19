use async_trait::async_trait;
use axum::{Json, RequestExt};
use axum::extract::{FromRequest, Request};
use validator::Validate;

use repo::dto::repository_dto::CreateUpdateRepoDto;

use crate::web::error::{ApiError, ApiResult};

pub struct CreateUpdateRepoDtoWrapper(pub CreateUpdateRepoDto);

#[async_trait]
impl<B> FromRequest<B> for CreateUpdateRepoDtoWrapper
    where
        B: Send,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, _state: &B) -> ApiResult<Self> {
        let Json(repo) = req
            .extract::<Json<CreateUpdateRepoDto>, _>()
            .await
            .map_err(|err| ApiError::Serialization(err))?;

        if let Err(errors) = repo.validate() {
            return Err(ApiError::Validation(errors));
        }
        Ok(CreateUpdateRepoDtoWrapper(repo))
    }
}