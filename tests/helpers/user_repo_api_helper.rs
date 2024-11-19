use axum_test::TestServer;
use ia_11_vorobei_ant::web::dto::user_repo_dto::{UserMultipleRepo, UserSingleRepo};
use repo::dto::DtoList;
use repo::dto::repo_dto::RepoDto;
use repo::utils::repository::repository_test_helper;
use repo::utils::user_repo::user_repo_test_helper;

pub async fn create_user_and_repo(client: &TestServer) -> UserSingleRepo {
    let (user_create_dto, repo_create_dto) = user_repo_test_helper::get_create_dtos();
    let user_res = client.post("/api/v1/users").json(&user_create_dto).await;
    let repo_res = client.post("/api/v1/repos").json(&repo_create_dto).await;
    UserSingleRepo::new(user_res.json(), repo_res.json())
}

pub async fn create_connected_user_and_repos(client: &TestServer) -> UserMultipleRepo {
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