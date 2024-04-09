use axum::http::StatusCode;
use axum_test::TestServer;
use mongodb::bson::oid::ObjectId;
use serial_test::serial;
use uuid::Uuid;

use ia_11_vorobei_ant::web::dto::user_repo_dto::UserMultipleRepo;
use repo::dto::repository_dto::RepoDto;
use repo::dto::user_dto::UserDto;
use repo::utils::repository::repository_test_helper;
use repo::utils::user_repo::user_repo_test_helper;

use crate::common::Setup;

#[tokio::test]
#[serial]
async fn add_pair_success() {
    let setup = Setup::new().await;
    let (user_id, repo_id) = create_user_and_repo(&setup.client).await;
    let expected_code = StatusCode::OK;

    let endpoint = format!("/apiV1/users/{user_id}/repo/{repo_id}");
    let res = setup.client.post(&endpoint).await;

    assert_eq!(res.status_code(), expected_code);
}

#[tokio::test]
#[serial]
async fn add_existing_pair_failure() {
    let setup = Setup::new().await;
    let (user_id, repo_id) = create_user_and_repo(&setup.client).await;
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
    let (user_id, repo_id) = create_user_and_repo(&setup.client).await;
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
    let (user_id, repo_id) = create_user_and_repo(&setup.client).await;
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
    let (user_id, repo_ids) = create_connected_user_and_repos(&setup.client).await;
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
    let (user_id, _) = create_connected_user_and_repos(&setup.client).await;
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

async fn create_user_and_repo(client: &TestServer) -> (ObjectId, Uuid) {
    let (user_create_dto, repo_create_dto) = user_repo_test_helper::get_create_dtos();
    let user_res = client.post("/apiV1/users").json(&user_create_dto).await;
    let repo_res = client.post("/apiV1/repos").json(&repo_create_dto).await;
    let UserDto { id: user_id, .. } = user_res.json();
    let RepoDto { id: repo_id, .. } = repo_res.json();
    (user_id.unwrap(), repo_id)
}

async fn create_connected_user_and_repos(client: &TestServer) -> (ObjectId, Vec<Uuid>) {
    let (user_id, repo_id) = create_user_and_repo(client).await;
    let create_repo_dtos = repository_test_helper::get_create_dtos();
    let mut repo_ids = vec![repo_id];
    for dto in create_repo_dtos {
        let repo_res = client.post("/apiV1/repos").json(&dto).await;
        let created_repo: RepoDto = repo_res.json();
        repo_ids.push(created_repo.id)
    }

    for repo_id in &repo_ids {
        let endpoint = format!("/apiV1/users/{user_id}/repo/{repo_id}");
        client.post(&endpoint).await;
    }

    (user_id, repo_ids)
}
