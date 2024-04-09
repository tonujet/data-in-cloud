use std::fmt::Display;
use std::sync::Arc;

use async_trait::async_trait;
use futures_util::StreamExt;
use mongodb::bson::oid::ObjectId;
use object_store::path::Path;
use object_store::ObjectStore;
use uuid::Uuid;

use error::RepoResult;

use crate::dao::error::Entity;
use crate::dao::error::RepoError::{AlreadyConnected, InternalConcrete, NotYetConnected};
use crate::dto::user_dto::UpdateUserDto;

use super::dto::DtoList;
use super::dto::{
    repository_dto::{CreateUpdateRepoDto, RepoDto},
    user_dto::{CreateUserDto, UserDto},
};

pub mod error;
pub mod repository_repo;
pub mod user_repo;
pub mod user_repository_repo;

#[async_trait]
pub trait RepoTrait<C, U, R, I>: Send + Sync {
    async fn create(&self, dto: C) -> RepoResult<R>;
    async fn update(&self, id: &I, dto: U) -> RepoResult<R>;
    async fn delete(&self, id: &I) -> RepoResult<R>;
    async fn get(&self, id: &I) -> RepoResult<R>;
    async fn list(&self, take: Option<u64>, offset: Option<u64>) -> RepoResult<DtoList<R>>;
}

pub trait RepositoryRepoTrait:
    RepoTrait<CreateUpdateRepoDto, CreateUpdateRepoDto, RepoDto, Uuid>
{
}

pub trait UserRepoTrait: RepoTrait<CreateUserDto, UpdateUserDto, UserDto, ObjectId> {}

const DELIMITER: &'static str = "____";

#[async_trait]
pub trait BlobConnRepoTrait<K, V>: Send + Sync
where
    K: Display + Send + Sync,
    V: Display + Send + Sync,
{
    async fn are_not_connected(&self, key_id: &K, val_id: &V) -> RepoResult<()>
    where
        Self: Sized,
    {
        let path = Path::from(format!("{key_id}{DELIMITER}{val_id}"));
        match self.store().head(&path).await {
            Ok(_) => Err(AlreadyConnected(Self::key_entity(), Self::val_entity())),
            Err(_) => Ok(()),
        }
    }

    async fn are_connected(&self, key_id: &K, val_id: &V) -> RepoResult<()>
    where
        Self: Sized,
    {
        match self.are_not_connected(key_id, val_id).await {
            Ok(_) =>  Err(NotYetConnected(Self::key_entity(), Self::val_entity())),
            Err(_) =>  Ok(())
        }
    }

    async fn is_val_connected(&self, val_id: &V) -> RepoResult<()>
        where
            Self: Sized,
    {
        let store = self.store();
        for item in store.list(None).collect::<Vec<_>>().await {
            let location = item?.location;
            let filename = location
                .filename()
                .ok_or(InternalConcrete(format!("Wrong location: {location}")))?;
            if filename.contains(&val_id.to_string()) {
                return Ok(())
            }
        }

        Err(NotYetConnected(Self::key_entity(), Self::val_entity()))
    }

    async fn is_val_not_connected(&self, val_id: &V) -> RepoResult<()>
        where
            Self: Sized,
    {
        match self.is_val_connected(val_id).await {
            Ok(_) =>  Err(AlreadyConnected(Self::key_entity(), Self::val_entity())),
            Err(_) =>  Ok(())
        }
    }
    
    fn get_file_name(&self, key_id: &K, val_id: &V) -> String {
        format!("{key_id}{DELIMITER}{val_id}")
    }
    async fn list_pairs(&self, key_id: &K) -> RepoResult<Vec<V>>;
    async fn add_pair(&self, key_id: &K, val_id: &V) -> RepoResult<()>;
    async fn delete_pair(&self, key_id: &K, val_id: &V) -> RepoResult<()>;

    async fn add_pairs(&self, key_id: &K, val_ids: Vec<&V>) -> RepoResult<()> {
        for val_id in val_ids {
            self.add_pair(key_id, val_id).await?
        }
        Ok(())
    }

    fn store(&self) -> Arc<dyn ObjectStore>;
    fn key_entity() -> Entity
    where
        Self: Sized;
    fn val_entity() -> Entity
    where
        Self: Sized;
}

pub trait UserRepositoryRepoTrait: BlobConnRepoTrait<ObjectId, Uuid> {}
