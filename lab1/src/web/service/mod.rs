use async_trait::async_trait;
use uuid::Uuid;

use super::error::{ApiError, ApiResult};
use repo::dto::{ListResponse, repository_dto::{CreateUpdateRepoDto, ResponseRepoDto}};

pub mod repository_service;

#[async_trait]
pub trait ServiceRepoTrait: Send + Sync {
    async fn create(&self, repo_dto: CreateUpdateRepoDto) -> ApiResult<ResponseRepoDto>;
    async fn update(&self, id: Uuid, repo_dto: CreateUpdateRepoDto) -> ApiResult<ResponseRepoDto>;
    async fn delete(&self, id: Uuid) -> ApiResult<ResponseRepoDto>;
    async fn get(&self, id: Uuid) -> ApiResult<ResponseRepoDto>;
    async fn list(&self, take: Option<u64>, offset: Option<u64>) -> ApiResult<ListResponse<ResponseRepoDto>>;
}