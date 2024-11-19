use crate::common::Setup;
use axum::http::StatusCode;
use serde_json::{json, Value};
use serial_test::serial;

mod api;
mod common;
pub mod helpers;

#[tokio::test]
#[serial]
async fn not_found_endpoint_success() {
    let setup = Setup::new().await;
    let expected_status_code = StatusCode::NOT_FOUND;
    let expected_body: Value = json!({
            "error_name": "EndpointNotFoundError",
            "message": "Endpoint http://localhost/not_found_endpoint not found",
            "status_code": "404",
            "status_code_message": "Not Found"
    });

    let res = setup.client.get("/not_found_endpoint").await;

    assert_eq!(res.status_code(), expected_status_code);
    assert_eq!(res.json::<Value>(), expected_body);
}
