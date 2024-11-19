use axum::http::StatusCode;
use axum_test::TestServer;
use serde_json::Value;
use serial_test::serial;

use repo::dto::{DtoList, repository_dto::RepoDto};
use repo::utils::repository::repository_test_helper;

use super::common::Setup;

#[tokio::test]
#[serial]
async fn create_repo_success() {
    let setup = Setup::new().await;
    let create_dto = repository_test_helper::get_create_dto();
    let expected_status_code = StatusCode::OK;
    let expected_body = repository_test_helper::get_response_from_create_dto();

    let res = setup.client.post("/apiV1/repos").json(&create_dto).await;
    let created_dto: RepoDto = res.json();

    assert_eq!(
        res.status_code(),
        expected_status_code,
        "Creation response status code doesn't correspond to the desired"
    );
    assert_eq!(
        created_dto, expected_body,
        "Creation response body of list doesn't correspond to the desired"
    );
}

#[tokio::test]
#[serial]
async fn create_repo_with_non_valid_data_failure() {
    let setup = Setup::new().await;
    let create_dto = repository_test_helper::get_invalid_create_update_dto();
    let expected_status_code = StatusCode::UNPROCESSABLE_ENTITY;
    let expected_body = repository_test_helper::get_response_from_invalid_dto();

    let res = setup.client.post("/apiV1/repos").json(&create_dto).await;

    assert_eq!(
        res.status_code(),
        expected_status_code,
        "Wrong creation response status code doesn't correspond to the desired"
    );
    assert_eq!(
        res.json::<Value>(),
        expected_body,
        "Wrong creation response body of list doesn't correspond to the desired"
    );
}

#[tokio::test]
#[serial]
async fn delete_repo_success() {
    let setup = Setup::new().await;
    let expected_status_code = StatusCode::CONFLICT;
    let expected_error_name = "RepositoryError";

    let deleted_dto = delete_repo(&setup.client).await;
    let res = setup
        .client
        .get(&format!("/apiV1/repos/{}", deleted_dto.id))
        .await;
    let error_name = &res.json::<Value>()["error_name"];

    assert_eq!(
        res.status_code(),
        expected_status_code,
        "Deletion response status code doesn't correspond to the desired"
    );
    assert_eq!(
        expected_error_name,
        error_name.as_str().unwrap(),
        "Deletion response body of list doesn't correspond to the desired"
    );
}

#[tokio::test]
#[serial]
async fn delete_deleted_repo_failure() {
    let setup = Setup::new().await;
    let expected_status_code = StatusCode::CONFLICT;
    let expected_error_name = "RepositoryError";

    let deleted_dto = delete_repo(&setup.client).await;
    let res = setup
        .client
        .delete(&format!("/apiV1/repos/{}", deleted_dto.id))
        .await;
    let error_name = &res.json::<Value>()["error_name"];

    assert_eq!(
        res.status_code(),
        expected_status_code,
        "Wrong deletion response status code doesn't correspond to the desired"
    );
    assert_eq!(
        expected_error_name,
        error_name.as_str().unwrap(),
        "Wrong deletion response body of list doesn't correspond to the desired"
    );
}

#[tokio::test]
#[serial]
async fn update_repo_success() {
    let setup = Setup::new().await;
    let update_dto = repository_test_helper::get_update_dto();
    let expected_status_code = StatusCode::OK;
    let expected_body = repository_test_helper::get_response_from_update_dto();

    let created_dto = create_repo(&setup.client).await;
    let res = setup
        .client
        .put(&format!("/apiV1/repos/{}", created_dto.id))
        .json(&update_dto)
        .await;

    assert_eq!(
        res.status_code(),
        expected_status_code,
        "Update response status code doesn't correspond to the desired"
    );
    assert_eq!(
        res.json::<RepoDto>(),
        expected_body,
        "Update response body of list doesn't correspond to the desired"
    );
}

#[tokio::test]
#[serial]
async fn update_repo_with_non_valid_data_failure() {
    let setup = Setup::new().await;
    let update_dto = repository_test_helper::get_invalid_create_update_dto();
    let expected_status_code = StatusCode::UNPROCESSABLE_ENTITY;
    let expected_body = repository_test_helper::get_response_from_invalid_dto();

    let created_dto = create_repo(&setup.client).await;
    let res = setup
        .client
        .put(&format!("/apiV1/repos/{}", { created_dto.id }))
        .json(&update_dto)
        .await;

    assert_eq!(
        res.status_code(),
        expected_status_code,
        "Wrong update response status code doesn't correspond to the desired"
    );
    assert_eq!(
        res.json::<Value>(),
        expected_body,
        "Wrong update response body of list doesn't correspond to the desired"
    );
}

#[tokio::test]
#[serial]
async fn update_deleted_repo_failure() {
    let setup = Setup::new().await;
    let expected_status_code = StatusCode::CONFLICT;
    let expected_error_name = "RepositoryError";
    let update_dto = repository_test_helper::get_update_dto();

    let deleted_dto = delete_repo(&setup.client).await;
    let res = setup
        .client
        .put(&format!("/apiV1/repos/{}", deleted_dto.id))
        .json(&update_dto)
        .await;
    let error_name = &res.json::<Value>()["error_name"];

    assert_eq!(
        res.status_code(),
        expected_status_code,
        "Wrong update response status code doesn't correspond to the desired"
    );
    assert_eq!(
        error_name,
        expected_error_name,
        "Wrong update response body of list doesn't correspond to the desired"
    );
}

#[tokio::test]
#[serial]
async fn get_repo_by_uuid_success() {
    let setup = Setup::new().await;
    let expected_status_code = StatusCode::OK;
    let expected_body = repository_test_helper::get_response_from_create_dto();

    let created_dto = create_repo(&setup.client).await;
    let res = setup
        .client
        .get(&format!("/apiV1/repos/{}", created_dto.id))
        .await;

    assert_eq!(
        res.status_code(),
        expected_status_code,
        "Get response status code doesn't correspond to the desired"
    );
    assert_eq!(
        res.json::<RepoDto>(),
        expected_body,
        "Get response body of list doesn't correspond to the desired"
    )
}

#[tokio::test]
#[serial]
async fn list_all_repos_success() {
    let setup = Setup::new().await;
    let response_dtos = repository_test_helper::get_response_from_create_dtos();
    let len = response_dtos.len();
    let expected_body = DtoList::new(response_dtos, len as u64, None, None);
    let expected_status_code = StatusCode::OK;

    create_some_repos(&setup.client).await;
    let res = setup.client.get("/apiV1/repos").await;

    assert_eq!(
        res.status_code(),
        expected_status_code,
        "List response status code doesn't correspond to the desired"
    );
    assert_eq!(
        res.json::<DtoList<RepoDto>>(),
        expected_body,
        "List response body doesn't correspond to the desired"
    )
}

#[tokio::test]
#[serial]
async fn list_repos_using_take_and_offset_success() {
    let setup = Setup::new().await;
    let dtos = repository_test_helper::get_response_from_create_dtos();
    let len = dtos.len();
    let offset = 2;
    let take = 2;
    let selected_dtos: Vec<RepoDto> = dtos.into_iter().skip(offset).take(take).collect();
    let expected_body = DtoList::new(
        selected_dtos,
        len as u64,
        Some(take as u64),
        Some(offset as u64),
    );
    let expected_status_code = StatusCode::OK;

    create_some_repos(&setup.client).await;
    let res = setup
        .client
        .get("/apiV1/repos")
        .add_query_param("take", 2)
        .add_query_param("offset", 2)
        .await;

    assert_eq!(
        res.status_code(),
        expected_status_code,
        "List response status code doesn't correspond to the desired"
    );
    assert_eq!(
        res.json::<DtoList<RepoDto>>(),
        expected_body,
        "List response body doesn't correspond to the desired"
    )
}

async fn delete_repo(client: &TestServer) -> RepoDto {
    let created_dto = create_repo(&client).await;
    let res = client
        .delete(&format!("/apiV1/repos/{}", created_dto.id))
        .await;
    let deleted_dto: RepoDto = res.json();
    deleted_dto
}

async fn create_repo(client: &TestServer) -> RepoDto {
    let create_dto = repository_test_helper::get_create_dto();
    let res = client.post("/apiV1/repos").json(&create_dto).await;
    let created_dto: RepoDto = res.json();
    created_dto
}


async fn create_some_repos(client: &TestServer) {
    let response_dtos = repository_test_helper::get_response_from_create_dtos();
    for (i, create_dto) in repository_test_helper::get_create_dtos()
        .iter()
        .enumerate()
    {
        let _ = &response_dtos[i];
        let _ = client.post("/apiV1/repos").json(&create_dto).await;
    }
}