use axum_test::TestServer;
use mongodb::bson::oid::ObjectId;
use uuid::Uuid;
use repo::dto::repo_dto::RepoDto;
use repo::dto::user_dto::UserDto;
use repo::utils::repository::repository_test_helper;
use repo::utils::user_repo::user_repo_test_helper;

pub async fn create_user_and_repo(client: &TestServer) -> (ObjectId, Uuid) {
    let (user_create_dto, repo_create_dto) = user_repo_test_helper::get_create_dtos();
    let user_res = client.post("/api/v1/users").json(&user_create_dto).await;
    let repo_res = client.post("/api/v1/repos").json(&repo_create_dto).await;
    let UserDto { id: user_id, .. } = user_res.json();
    let RepoDto { id: repo_id, .. } = repo_res.json();
    (user_id.unwrap(), repo_id)
}

pub async fn create_connected_user_and_repos(client: &TestServer) -> (ObjectId, Vec<Uuid>) {
    let (user_id, repo_id) = create_user_and_repo(client).await;
    let create_repo_dtos = repository_test_helper::get_create_dtos();
    let mut repo_ids = vec![repo_id];
    for dto in create_repo_dtos {
        let repo_res = client.post("/api/v1/repos").json(&dto).await;
        let created_repo: RepoDto = repo_res.json();
        repo_ids.push(created_repo.id)
    }

    for repo_id in &repo_ids {
        let endpoint = format!("/api/v1/users/{user_id}/repos/{repo_id}");
        client.post(&endpoint).await;
    }

    (user_id, repo_ids)
}