use sea_orm::DatabaseBackend;
use serial_test::serial;

use crate::utils::repository::repository_test_helper;
use crate::dao::RepoTrait;

use super::RepositoryRepo;
use super::RepositoryRepoTrait;

pub async fn get_stub_repo() -> impl RepositoryRepoTrait {
    repository_test_helper::get_model();
    let alive_repo_dto = repository_test_helper::get_model();

    let mut deleted_repo_dto = alive_repo_dto.clone();
    deleted_repo_dto.deleted = true;

    let conn = sea_orm::MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([
            [alive_repo_dto.clone()],
            [alive_repo_dto],
            [deleted_repo_dto.clone()],
            [deleted_repo_dto],
        ])
        .into_connection();

    RepositoryRepo::new(conn)
}

#[tokio::test]
#[serial]
async fn it_should_not_update_deleted_repo() {
    let repository_repo = get_stub_repo().await;
    let create_repo = repository_test_helper::get_create_dto();
    
    let repo = repository_repo.create(create_repo).await.unwrap();
    let repo = repository_repo.delete(&repo.id).await.unwrap();
    let update_repo = repository_test_helper::get_update_dto();
    let repo = repository_repo.update(&repo.id, update_repo).await;

    assert!(
        repo.is_err(),
        "Status code of update non-existing repository is not equal to the desired"
    );
}

#[tokio::test]
#[serial]
async fn it_should_not_delete_deleted_repo() {
    let repository_repo = get_stub_repo().await;
    let create_repo = repository_test_helper::get_create_dto();

    let repo = repository_repo.create(create_repo).await.unwrap();
    let repo = repository_repo.delete(&repo.id).await.unwrap();
    let repo = repository_repo.delete(&repo.id).await;

    assert!(
        repo.is_err(),
        "Status code of deletion non-existing repository is not equal to the desired"
    );
}