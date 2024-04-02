pub mod validation;

use axum::body::Body;
use axum::extract::{Host, Request};
use axum::response::IntoResponse;

use crate::web::error::ApiError;

pub async fn api_not_found_handler(Host(hostname): Host, req: Request<Body>) -> impl IntoResponse {
    ApiError::EndpointNotFound {
        hostname,
        uri: req.uri().to_string(),
    }
}
