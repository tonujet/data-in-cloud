use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use repo::dao::RepoRepositoryTrait;

use super::{ApiResult, RepoServiceTrait};
use super::DtoList;
use super::ServiceTrait;
use super::{CreateUpdateRepoDto, RepoDto};

#[derive(Clone)]
pub struct RepositoryService {
    repo: Arc<dyn RepoRepositoryTrait>,
}

#[async_trait]
impl ServiceTrait<CreateUpdateRepoDto, CreateUpdateRepoDto, RepoDto, Uuid> for RepositoryService {
    async fn create(&self, repo_dto: CreateUpdateRepoDto) -> ApiResult<RepoDto> {
        Ok(self.repo.create(repo_dto).await?)
    }

    async fn update(&self, id: &Uuid, repo_dto: CreateUpdateRepoDto) -> ApiResult<RepoDto> {
        Ok(self.repo.update(id, repo_dto).await?)
    }

    async fn delete(&self, id: &Uuid) -> ApiResult<RepoDto> {
        Ok(self.repo.delete(id).await?)
    }

    async fn get(&self, id: &Uuid) -> ApiResult<RepoDto> {
        Ok(self.repo.get(id).await?)
    }

    async fn list(
        &self,
        take: Option<u64>,
        offset: Option<u64>,
    ) -> ApiResult<DtoList<RepoDto>> {
        Ok(self.repo.list(take, offset).await?)
    }
}

impl RepoServiceTrait for RepositoryService{}


impl RepositoryService {
    pub fn new(repo: Arc<dyn RepoRepositoryTrait>) -> Self {
        Self { repo }
    }
}
