use std::fmt::Debug;
use async_trait::async_trait;
use axum::{Json, RequestExt};
use axum::extract::{FromRequest, Request};
use serde::de::DeserializeOwned;
use validator::Validate;


use crate::web::error::{ApiError, ApiResult};

pub struct ValidationWrapper<T: DeserializeOwned + Debug + Validate + 'static >(pub  T);


#[async_trait]
impl<T, B> FromRequest<B> for ValidationWrapper<T>
    where
        B: Send,
        T: DeserializeOwned + Debug + Validate + 'static
{
    type Rejection = ApiError;

    async fn from_request(req: Request, _state: &B) -> ApiResult<Self> {
        let Json(dto) = req
            .extract::<Json<T>, _>()
            .await?;

        if let Err(errors) = dto.validate() {
            return Err(ApiError::Validation(errors));
        }
        Ok(ValidationWrapper(dto))
    }
}