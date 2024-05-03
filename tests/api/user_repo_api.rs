use axum::http::StatusCode;
use serial_test::serial;

use ia_11_vorobei_ant::web::dto::user_repo_dto::UserMultipleRepo;

use crate::common::Setup;
use crate::helpers::user_repo_api_helper;

#[tokio::test]
#[serial]
async fn add_pair_success() {
    let setup = Setup::new().await;
    let (user_id, repo_id) = user_repo_api_helper::create_user_and_repo(&setup.client).await;
    let expected_code = StatusCode::OK;

    let endpoint = format!("/apiV1/users/{user_id}/repo/{repo_id}");
    let res = setup.client.post(&endpoint).await;

    assert_eq!(res.status_code(), expected_code);
}

#[tokio::test]
#[serial]
async fn add_existing_pair_failure() {
    let setup = Setup::new().await;
    let (user_id, repo_id) = user_repo_api_helper::create_user_and_repo(&setup.client).await;
    let expected_code = StatusCode::CONFLICT;

    let endpoint = format!("/apiV1/users/{user_id}/repo/{repo_id}");
    setup.client.post(&endpoint).await;
    let res = setup.client.post(&endpoint).await;

    assert_eq!(res.status_code(), expected_code);
}

#[tokio::test]
#[serial]
async fn delete_pair_success() {
    let setup = Setup::new().await;
    let (user_id, repo_id) = user_repo_api_helper::create_user_and_repo(&setup.client).await;
    let expected_code = StatusCode::OK;

    let endpoint = format!("/apiV1/users/{user_id}/repo/{repo_id}");
    setup.client.post(&endpoint).await;
    let res = setup.client.delete(&endpoint).await;

    assert_eq!(res.status_code(), expected_code);
}

#[tokio::test]
#[serial]
async fn delete_two_times_the_same_pair_failure() {
    let setup = Setup::new().await;
    let (user_id, repo_id) = user_repo_api_helper::create_user_and_repo(&setup.client).await;
    let expected_code = StatusCode::CONFLICT;

    let endpoint = format!("/apiV1/users/{user_id}/repo/{repo_id}");
    setup.client.post(&endpoint).await;
    setup.client.delete(&endpoint).await;
    let res = setup.client.delete(&endpoint).await;

    assert_eq!(res.status_code(), expected_code);
}

#[tokio::test]
#[serial]
async fn list_all_pairs_success() {
    let setup = Setup::new().await;
    let (user_id, repo_ids) = user_repo_api_helper::create_connected_user_and_repos(&setup.client).await;
    let expected_code = StatusCode::OK;
    let expected_len = repo_ids.len();

    let endpoint = format!("/apiV1/users/{user_id}/repo");
    let res = setup.client.get(&endpoint).await;
    let user_repos_dto: UserMultipleRepo = res.json();

    assert_eq!(res.status_code(), expected_code);
    assert_eq!(user_repos_dto.repos.dtos.len(), expected_len);
}

#[tokio::test]
#[serial]
async fn list_pairs_with_pagination_success() {
    let setup = Setup::new().await;
    let (user_id, _) = user_repo_api_helper::create_connected_user_and_repos(&setup.client).await;
    let take = 3;
    let offset = 2;
    let expected_code = StatusCode::OK;
    let expected_len = take;

    let endpoint = format!("/apiV1/users/{user_id}/repo");
    let res = setup
        .client
        .get(&endpoint)
        .add_query_param("take", take)
        .add_query_param("offset", offset)
        .await;
    let user_repos_dto: UserMultipleRepo = res.json();

    assert_eq!(res.status_code(), expected_code);
    assert_eq!(user_repos_dto.repos.dtos.len(), expected_len);
}


