use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use repo::dao::error::RepoError;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;
use thiserror::Error;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, AsRefStr, Error)]
pub enum ApiError {
    #[error(transparent)]
    Repository(#[from] RepoError),

    #[error("{}", .0.body_text())]
    Serialization(#[from] JsonRejection),

    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),

    #[error(transparent)]
    MessageBroker(#[from] message_broker::error::MBrokerError),

    #[error("Endpoint {uri} not found")]
    EndpointNotFound { hostname: String, uri: String },
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::Validation(ref errs) => self.to_response(StatusCode::UNPROCESSABLE_ENTITY, errs),

            Self::Serialization(_) => {
                self.to_response(StatusCode::UNPROCESSABLE_ENTITY, self.to_string())
            }

            Self::EndpointNotFound { .. } => {
                self.to_response(StatusCode::NOT_FOUND, self.to_string())
            }

            Self::Repository(ref err) => match err {
                RepoError::SqlExecution(_)
                | RepoError::MongoExecution(_)
                | RepoError::ObjectStore(_) => {
                    eprintln!("{err}");
                    self.to_internal_error()
                }
                _ => self.to_response(StatusCode::CONFLICT, self.to_string()),
            },
            Self::MessageBroker(_) => self.to_internal_error(),
        }
    }
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct ApiErrorResponse<'a, T: Serialize> {
    #[schema(example = 400)]
    status_code: u16,

    #[schema(example = "BAD REQUEST")]
    status_code_message: &'a str,

    message: T,

    #[schema(example = "ClientError")]
    error_name: &'a str,
}

impl ApiError {
    fn to_response<T: Serialize>(&self, code: StatusCode, message: T) -> Response {
        let response = ApiErrorResponse {
            message,
            status_code: code.as_u16(),
            status_code_message: code.canonical_reason().unwrap_or("Unknown"),
            error_name: &format!("{}Error", self.as_ref()),
        };

        (code, Json(response)).into_response()
    }

    fn to_internal_error(&self) -> Response {
        self.to_response(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
    }
}

impl async_graphql::ErrorExtensions for ApiError {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(self.to_string())
            .extend_with(|_, e| e.set("error_name", format!("{}Error", self.as_ref())))
    }
}
