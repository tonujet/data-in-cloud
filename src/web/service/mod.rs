use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use uuid::Uuid;

use dto::user_dto::{CreateUserDto, UpdateUserDto, UserDto};
use dto::user_repo_info_dto::{CreateUserRepoInfoDto, UserRepoInfoDto};
use dto::{
    repo_dto::{CreateUpdateRepoDto, RepoDto},
    DtoList, OneToManyDto, OneToOneDto,
};

use super::error::ApiResult;

pub mod repo_service;
pub mod user_repo_info_receiver;
pub mod user_repo_info_service;
pub mod user_repo_service;
pub mod user_service;

#[async_trait]
pub trait ServiceTrait<C, U, R, I>: Send + Sync
where
    R: async_graphql::OutputType + utoipa::ToSchema,
{
    async fn create(&self, dto: C) -> ApiResult<R>;
    async fn update(&self, id: &I, dto: U) -> ApiResult<R>;
    async fn delete(&self, id: &I) -> ApiResult<R>;
    async fn get(&self, id: &I) -> ApiResult<R>;
    async fn list(&self, take: Option<u64>, offset: Option<u64>) -> ApiResult<DtoList<R>>;
}

#[async_trait]
pub trait UserServiceTrait: ServiceTrait<CreateUserDto, UpdateUserDto, UserDto, ObjectId> {
    async fn list_user_repos_info(
        &self,
        id: ObjectId,
        page: Option<u64>,
        offset: Option<u64>,
    ) -> ApiResult<DtoList<UserRepoInfoDto>>;
}

pub trait RepoServiceTrait:
    ServiceTrait<CreateUpdateRepoDto, CreateUpdateRepoDto, RepoDto, Uuid>
{
}

#[async_trait]
pub trait PersistentServiceTrait<C, R, I>: Send + Sync
where
    R: async_graphql::OutputType + utoipa::ToSchema,
{
    async fn create(&self, dto: C) -> ApiResult<R>;
    async fn get(&self, id: &I) -> ApiResult<R>;
    async fn list(&self, take: Option<u64>, offset: Option<u64>) -> ApiResult<DtoList<R>>;
}

#[async_trait]
pub trait UserRepoInfoServiceTrait:
    PersistentServiceTrait<CreateUserRepoInfoDto, UserRepoInfoDto, ObjectId>
{
}

#[async_trait]
pub trait BlobConnServiceTrait<K, V, S, M>: Send + Sync {
    async fn add_pair(&self, key_id: &K, val_id: &V) -> ApiResult<S>;
    async fn list_pairs(&self, key_id: &K, take: Option<u64>, offset: Option<u64>) -> ApiResult<M>;
    async fn delete_pair(&self, key_id: &K, val_id: &V) -> ApiResult<S>;
}

pub trait UserRepoServiceTrait:
    BlobConnServiceTrait<ObjectId, Uuid, OneToOneDto<UserDto, RepoDto>, OneToManyDto<UserDto, RepoDto>>
{
}
