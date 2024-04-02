use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use uuid::Uuid;

use error::RepoResult;

use crate::dto::user_dto::UpdateUserDto;

use super::dto::DtoList;
use super::dto::{
    repository_dto::{CreateUpdateRepoDto, RepoDto},
    user_dto::{CreateUserDto, UserDto},
};

pub mod error;
pub mod repository_repo;
pub mod user_repo;

#[async_trait]
pub trait RepoTrait<C, U, R, I>: Send + Sync {
    async fn create(&self, dto: C) -> RepoResult<R>;
    async fn update(&self, id: I, dto: U) -> RepoResult<R>;
    async fn delete(&self, id: I) -> RepoResult<R>;
    async fn get(&self, id: I) -> RepoResult<R>;
    async fn list(&self, take: Option<u64>, offset: Option<u64>) -> RepoResult<DtoList<R>>;
}

pub trait RepositoryRepoTrait:
    RepoTrait<CreateUpdateRepoDto, CreateUpdateRepoDto, RepoDto, Uuid>
{
}

pub trait UserRepoTrait: RepoTrait<CreateUserDto, UpdateUserDto, UserDto, ObjectId> {}
