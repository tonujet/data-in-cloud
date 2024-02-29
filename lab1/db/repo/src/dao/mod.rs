use async_trait::async_trait;
use uuid::Uuid;

use error::RepoResult;

use super::dto::ListResponse;
use super::dto::repository_dto::{CreateUpdateRepoDto, ResponseRepoDto};
use super::utils;

pub mod error;
pub mod repository_repo;


#[async_trait]
pub trait RepositoryRepoTrait: Send + Sync {
    async fn create(&self, repo_dto: CreateUpdateRepoDto) -> RepoResult<ResponseRepoDto>;
    async fn update(&self, id: Uuid, repo_dto: CreateUpdateRepoDto) -> RepoResult<ResponseRepoDto>;
    async fn delete(&self, id: Uuid) -> RepoResult<ResponseRepoDto>;
    async fn get(&self, id: Uuid) -> RepoResult<ResponseRepoDto>;
    async fn list(
        &self,
        take: Option<u64>,
        offset: Option<u64>,
    ) -> RepoResult<ListResponse<ResponseRepoDto>>;
}
