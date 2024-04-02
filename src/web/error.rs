use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use serde_json::json;
use strum::AsRefStr;
use thiserror::Error;

use repo::dao::error::RepoError;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, AsRefStr, Error)]
pub enum ApiError {
    #[error(transparent)]
    Repository(#[from] RepoError),

    #[error("{}", .0.body_text())]
    Serialization(#[from] JsonRejection),

    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),

    #[error("Endpoint {uri} not found")]
    EndpointNotFound { hostname: String, uri: String },
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        return match self {
            Self::Validation(ref errs) => self.to_response(StatusCode::UNPROCESSABLE_ENTITY, errs),

            Self::Serialization(_) => {
                self.to_response(StatusCode::UNPROCESSABLE_ENTITY, self.to_string())
            }

            Self::EndpointNotFound { .. } => {
                self.to_response(StatusCode::NOT_FOUND, self.to_string())
            }

            Self::Repository(ref err) => match err {
                RepoError::SqlExecution(_) | RepoError::MongoExecution(_) => self.to_internal_error(),
                _ => self.to_response(StatusCode::CONFLICT, self.to_string()),
            },
        };
    }
}

impl ApiError {
    fn to_response<T: Serialize>(&self, code: StatusCode, message: T) -> Response {
        let json = json!({
            "status_code": code.as_str(),
            "status_code_message": code.canonical_reason().unwrap_or("Unknown"),
            "message": message,
            "error_name": format!("{}Error", self.as_ref())
        });

        (code, Json(json)).into_response()
    }

    fn to_internal_error(&self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}
