use axum::http::StatusCode;
use serial_test::serial;

use ia_11_vorobei_ant::web::dto::user_repo_dto::{UserMultipleRepo, UserSingleRepo};
use repo::dto::DtoList;

use crate::common::Setup;
use crate::helpers::user_repo_api_helper;

#[tokio::test]
#[serial]
async fn add_pair_success() {
    let setup = Setup::new().await;
    let UserSingleRepo {user, repo} = user_repo_api_helper::create_user_and_repo(&setup.client).await;
    let expected_code = StatusCode::OK;
    let expected_body = UserSingleRepo::new(user.clone(), repo.clone());

    let endpoint = format!("/api/v1/users/{}/repos/{}", user.id.unwrap(), repo.id);
    let res = setup.client.post(&endpoint).await;

    assert_eq!(res.status_code(), expected_code);
    assert_eq!(res.json::<UserSingleRepo>(), expected_body);
}

#[tokio::test]
#[serial]
async fn add_existing_pair_failure() {
    let setup = Setup::new().await;
    let UserSingleRepo {user, repo}  = user_repo_api_helper::create_user_and_repo(&setup.client).await;
    let expected_code = StatusCode::CONFLICT;

    let endpoint = format!("/api/v1/users/{}/repos/{}", user.id.unwrap(), repo.id);
    setup.client.post(&endpoint).await;
    let res = setup.client.post(&endpoint).await;

    assert_eq!(res.status_code(), expected_code);
}

#[tokio::test]
#[serial]
async fn delete_pair_success() {
    let setup = Setup::new().await;
    let UserSingleRepo {user, repo}  = user_repo_api_helper::create_user_and_repo(&setup.client).await;
    let expected_code = StatusCode::OK;
    let expected_body = UserSingleRepo::new(user.clone(), repo.clone());

    let endpoint = format!("/api/v1/users/{}/repos/{}", user.id.unwrap(), repo.id);
    setup.client.post(&endpoint).await;
    let res = setup.client.delete(&endpoint).await;

    assert_eq!(res.status_code(), expected_code);
    assert_eq!(res.json::<UserSingleRepo>(), expected_body);
}

#[tokio::test]
#[serial]
async fn delete_two_times_the_same_pair_failure() {
    let setup = Setup::new().await;
    let UserSingleRepo {user, repo}  = user_repo_api_helper::create_user_and_repo(&setup.client).await;
    let expected_code = StatusCode::CONFLICT;

    let endpoint = format!("/api/v1/users/{}/repos/{}", user.id.unwrap(), repo.id);
    setup.client.post(&endpoint).await;
    setup.client.delete(&endpoint).await;
    let res = setup.client.delete(&endpoint).await;

    assert_eq!(res.status_code(), expected_code);
}

#[tokio::test]
#[serial]
async fn list_all_pairs_success() {
    let setup = Setup::new().await;
    let user_multiple_repos: UserMultipleRepo = user_repo_api_helper::create_connected_user_and_repos(&setup.client).await;
    let expected_code = StatusCode::OK;

    let endpoint = format!("/api/v1/users/{}/repos", user_multiple_repos.user.id.unwrap());
    let res = setup.client.get(&endpoint).await;

    assert_eq!(res.status_code(), expected_code);
    assert_eq!(res.json::<UserMultipleRepo>(), user_multiple_repos)
}

#[tokio::test]
#[serial]
async fn list_pairs_with_pagination_success() {
    let setup = Setup::new().await;
    let UserMultipleRepo {user, repos} = user_repo_api_helper::create_connected_user_and_repos(&setup.client).await;
    let take = 3;
    let offset = 2;
    let expected_code = StatusCode::OK;
    let repos_len = repos.dtos.len() as u64;
    let expected_body = UserMultipleRepo::new(user.clone(), DtoList::new(repos.dtos.into_iter().skip(offset).take(take).collect(), repos_len, Some(take as u64), Some(offset as u64)));

    let endpoint = format!("/api/v1/users/{}/repos", user.id.unwrap());
    let res = setup
        .client
        .get(&endpoint)
        .add_query_param("take", take)
        .add_query_param("offset", offset)
        .await;

    assert_eq!(res.status_code(), expected_code);
    assert_eq!(res.json::<UserMultipleRepo>(), expected_body);
}


