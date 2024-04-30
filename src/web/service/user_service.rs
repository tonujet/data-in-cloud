use std::sync::Arc;
use async_trait::async_trait;

use mongodb::bson::oid::ObjectId;

use repo::dao::UserRepositoryTrait;
use repo::dto::user_dto::{CreateUserDto, UpdateUserDto, UserDto};
use repo::dto::DtoList;
use repo::dto::user_repo_info_dto::UserRepoInfoDto;

use crate::web::error::ApiResult;
use crate::web::service::{ServiceTrait, UserRepoInfoServiceTrait, UserServiceTrait};
use crate::web::service::user_repo_info_service::UserRepoInfoService;

pub struct UserService {
    repo: Arc<dyn UserRepositoryTrait>,
    user_repo_info_service: Arc<dyn UserRepoInfoServiceTrait>
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepositoryTrait>, user_repo_info_service: Arc<dyn UserRepoInfoServiceTrait>) -> Self {
        UserService {
            repo,
            user_repo_info_service
        }
    }
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn list_user_repos_info(&self, id: ObjectId, page: Option<u64>, offset: Option<u64>) -> ApiResult<DtoList<UserRepoInfoDto>> {
        Ok(self.user_repo_info_service.list_by_user_id(id, page, offset).await?)
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
