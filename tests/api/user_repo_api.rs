use axum::http::StatusCode;
use axum_test::TestServer;
use serial_test::serial;

use ia_11_vorobei_ant::web::dto::user_repo_dto::{UserMultipleRepo, UserSingleRepo};
use repo::dto::DtoList;
use repo::dto::repository_dto::RepoDto;
use repo::utils::repository::repository_test_helper;
use repo::utils::user_repo::user_repo_test_helper;

use crate::common::Setup;

#[tokio::test]
#[serial]
async fn add_pair_success() {
    let setup = Setup::new().await;
    let UserSingleRepo {user, repo} = create_user_and_repo(&setup.client).await;
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
    let UserSingleRepo {user, repo}  = create_user_and_repo(&setup.client).await;
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
    let UserSingleRepo {user, repo}  = create_user_and_repo(&setup.client).await;
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
    let UserSingleRepo {user, repo}  = create_user_and_repo(&setup.client).await;
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
    let user_multiple_repos: UserMultipleRepo = create_connected_user_and_repos(&setup.client).await;
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
    let UserMultipleRepo {user, repos} = create_connected_user_and_repos(&setup.client).await;
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

async fn create_user_and_repo(client: &TestServer) -> UserSingleRepo {
    let (user_create_dto, repo_create_dto) = user_repo_test_helper::get_create_dtos();
    let user_res = client.post("/api/v1/users").json(&user_create_dto).await;
    let repo_res = client.post("/api/v1/repos").json(&repo_create_dto).await;
    UserSingleRepo::new(user_res.json(), repo_res.json())
}

async fn create_connected_user_and_repos(client: &TestServer) -> UserMultipleRepo {
    let UserSingleRepo {user: user_dto, repo: repo_dto}  = create_user_and_repo(client).await;
    let create_repo_dtos = repository_test_helper::get_create_dtos();
    let mut repo_dtos = vec![repo_dto];
    for dto in create_repo_dtos {
        let repo_res = client.post("/api/v1/repos").json(&dto).await;
        let created_repo: RepoDto = repo_res.json();
        repo_dtos.push(created_repo)
    }

    for repo_dto in &repo_dtos {
        let endpoint = format!("/api/v1/users/{}/repos/{}", user_dto.id.unwrap(), repo_dto.id);
        client.post(&endpoint).await;
    }

    let dto_len = repo_dtos.len() as u64;
    repo_dtos.reverse();
    UserMultipleRepo::new(user_dto, DtoList::new(repo_dtos, dto_len, Some(dto_len), None))
}
