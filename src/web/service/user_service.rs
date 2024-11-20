use async_trait::async_trait;
use std::sync::Arc;

use mongodb::bson::oid::ObjectId;

use dto::user_dto::{CreateUserDto, UpdateUserDto, UserDto};
use dto::user_repo_info_dto::UserRepoInfoDto;
use dto::DtoList;
use repo::dao::{UserRepoInfoRepositoryTrait, UserRepositoryTrait};

use crate::web::error::ApiResult;
use crate::web::service::{ServiceTrait, UserServiceTrait};

pub struct UserService {
    repo: Arc<dyn UserRepositoryTrait>,
    user_repo_info_repo: Arc<dyn UserRepoInfoRepositoryTrait>,
}

impl UserService {
    pub fn new(
        repo: Arc<dyn UserRepositoryTrait>,
        user_repo_info_repo: Arc<dyn UserRepoInfoRepositoryTrait>,
    ) -> Self {
        Self {
            repo,
            user_repo_info_repo,
        }
    }
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn list_user_repos_info(
        &self,
        id: ObjectId,
        page: Option<u64>,
        offset: Option<u64>,
    ) -> ApiResult<DtoList<UserRepoInfoDto>> {
        Ok(self
            .user_repo_info_repo
            .list_by_user_id(id, page, offset)
            .await?)
    }
}

#[async_trait]
impl ServiceTrait<CreateUserDto, UpdateUserDto, UserDto, ObjectId> for UserService {
    async fn create(&self, dto: CreateUserDto) -> ApiResult<UserDto> {
        Ok(self.repo.create(dto).await?)
    }

    async fn update(&self, id: &ObjectId, dto: UpdateUserDto) -> ApiResult<UserDto> {
        Ok(self.repo.update(id, dto).await?)
    }

    async fn delete(&self, id: &ObjectId) -> ApiResult<UserDto> {
        Ok(self.repo.delete(id).await?)
    }

    async fn get(&self, id: &ObjectId) -> ApiResult<UserDto> {
        Ok(self.repo.get(id).await?)
    }

    async fn list(&self, take: Option<u64>, offset: Option<u64>) -> ApiResult<DtoList<UserDto>> {
        Ok(self.repo.list(take, offset).await?)
    }
}
