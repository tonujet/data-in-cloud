use std::sync::Arc;
use async_trait::async_trait;

use mongodb::bson::oid::ObjectId;

use repo::dao::UserRepoTrait;
use repo::dto::user_dto::{CreateUserDto, UpdateUserDto, UserDto};
use repo::dto::DtoList;

use crate::web::error::ApiResult;
use crate::web::service::{ServiceTrait, UserServiceTrait};

pub struct UserService {
    repo: Arc<dyn UserRepoTrait>,
}

impl UserService {
    pub(crate) fn new(repo: Arc<dyn UserRepoTrait>) -> Self {
        UserService {
            repo
        }
    }
}

impl UserServiceTrait for UserService {}

#[async_trait]
impl ServiceTrait<CreateUserDto, UpdateUserDto, UserDto, ObjectId> for UserService {
    async fn create(&self, dto: CreateUserDto) -> ApiResult<UserDto> {
        Ok(self.repo.create(dto).await?)
    }

    async fn update(&self, id: ObjectId, dto: UpdateUserDto) -> ApiResult<UserDto> {
        Ok(self.repo.update(id, dto).await?)
    }

    async fn delete(&self, id: ObjectId) -> ApiResult<UserDto> {
        Ok(self.repo.delete(id).await?)
    }

    async fn get(&self, id: ObjectId) -> ApiResult<UserDto> {
        Ok(self.repo.get(id).await?)
    }

    async fn list(&self, take: Option<u64>, offset: Option<u64>) -> ApiResult<DtoList<UserDto>> {
        Ok(self.repo.list(take, offset).await?)
    }
}
