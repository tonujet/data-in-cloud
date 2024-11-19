use axum_test::TestServer;
use repo::dto::repo_dto::RepoDto;
use repo::utils::repository::repository_test_helper;

pub async fn delete_repo(client: &TestServer) -> RepoDto {
    let created_dto = create_repo(&client).await;
    let res = client
        .delete(&format!("/api/v1/repos/{}", created_dto.id))
        .await;
    let deleted_dto: RepoDto = res.json();
    deleted_dto
}

pub async fn create_repo(client: &TestServer) -> RepoDto {
    let create_dto = repository_test_helper::get_create_dto();
    let res = client.post("/api/v1/repos").json(&create_dto).await;
    let created_dto: RepoDto = res.json();
    created_dto
}


pub async fn create_some_repos(client: &TestServer) {
    let response_dtos = repository_test_helper::get_response_from_create_dtos();
    for (i, create_dto) in repository_test_helper::get_create_dtos()
        .iter()
        .enumerate()
    {
        let _ = &response_dtos[i];
        let _ = client.post("/api/v1/repos").json(&create_dto).await;
    }
}