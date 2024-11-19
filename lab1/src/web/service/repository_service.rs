use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use repo::dao::RepositoryRepoTrait;

use super::{ApiError, ApiResult};
use super::{CreateUpdateRepoDto, ResponseRepoDto};
use super::ListResponse;
use super::ServiceRepoTrait;

#[derive(Clone)]
pub struct RepositoryService {
    repo: Arc<dyn RepositoryRepoTrait>,
}

#[async_trait]
impl ServiceRepoTrait for RepositoryService {
    async fn create(&self, repo_dto: CreateUpdateRepoDto) -> ApiResult<ResponseRepoDto> {
        self.repo
            .create(repo_dto)
            .await
            .map_err(|err| ApiError::Repository(err))
    }

    async fn update(&self, id: Uuid, repo_dto: CreateUpdateRepoDto) -> ApiResult<ResponseRepoDto> {
        self.repo
            .update(id, repo_dto)
            .await
            .map_err(|err| ApiError::Repository(err))
    }

    async fn delete(&self, id: Uuid) -> ApiResult<ResponseRepoDto> {
        self.repo
            .delete(id)
            .await
            .map_err(|err| ApiError::Repository(err))
    }

    async fn get(&self, id: Uuid) -> ApiResult<ResponseRepoDto> {
        self.repo
            .get(id)
            .await
            .map_err(|err| ApiError::Repository(err))
    }

    async fn list(
        &self,
        take: Option<u64>,
        offset: Option<u64>,
    ) -> ApiResult<ListResponse<ResponseRepoDto>> {
        self.repo
            .list(take, offset)
            .await
            .map_err(|err| ApiError::Repository(err))
    }
}

impl RepositoryService {
    pub fn new(repo: Arc<dyn RepositoryRepoTrait>) -> Self {
        Self { repo }
    }
}
