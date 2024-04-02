use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use uuid::Uuid;

use super::error::{ApiResult};
use repo::dto::{DtoList, repository_dto::{CreateUpdateRepoDto, RepoDto}};
use repo::dto::user_dto::{CreateUserDto, UpdateUserDto, UserDto};

pub mod repository_service;
pub mod user_service;

#[async_trait]
pub trait ServiceTrait<C, U, R, I>: Send + Sync {
    async fn create(&self, dto: C) -> ApiResult<R>;
    async fn update(&self, id: I, dto: U) -> ApiResult<R>;
    async fn delete(&self, id: I) -> ApiResult<R>;
    async fn get(&self, id: I) -> ApiResult<R>;
    async fn list(&self, take: Option<u64>, offset: Option<u64>) -> ApiResult<DtoList<R>>;
}

pub trait UserServiceTrait : ServiceTrait<CreateUserDto, UpdateUserDto, UserDto, ObjectId> {}

pub trait RepoServiceTrait: ServiceTrait<CreateUpdateRepoDto, CreateUpdateRepoDto, RepoDto, Uuid> {}