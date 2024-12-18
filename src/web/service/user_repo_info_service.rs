use std::sync::Arc;

use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;

use dto::user_repo_info_dto::{CreateUserRepoInfoDto, UserRepoInfoDto};
use dto::DtoList;
use repo::dao::UserRepoInfoRepositoryTrait;

use crate::web::error::ApiResult;
use crate::web::service::{PersistentServiceTrait, UserRepoInfoServiceTrait};

#[derive(Clone)]
pub struct UserRepoInfoService {
    repo: Arc<dyn UserRepoInfoRepositoryTrait>,
}

impl UserRepoInfoService {
    pub fn new(repo: Arc<dyn UserRepoInfoRepositoryTrait>) -> Self {
        Self { repo }
    }
}

impl UserRepoInfoServiceTrait for UserRepoInfoService {}
#[async_trait]
impl PersistentServiceTrait<CreateUserRepoInfoDto, UserRepoInfoDto, ObjectId>
    for UserRepoInfoService
{
    async fn create(&self, dto: CreateUserRepoInfoDto) -> ApiResult<UserRepoInfoDto> {
        Ok(self.repo.create(dto).await?)
    }

    async fn get(&self, id: &ObjectId) -> ApiResult<UserRepoInfoDto> {
        Ok(self.repo.get(id).await?)
    }

    async fn list(
        &self,
        take: Option<u64>,
        offset: Option<u64>,
    ) -> ApiResult<DtoList<UserRepoInfoDto>> {
        Ok(self.repo.list(take, offset).await?)
    }
}
