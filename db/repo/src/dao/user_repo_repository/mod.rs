#[cfg(test)]
mod tests;

use async_trait::async_trait;
use std::cmp::Reverse;
use std::str::FromStr;
use std::sync::Arc;

use crate::dao::{BlobConnRepositoryTrait, UserRepoRepositoryTrait, DELIMITER};
use bytes::Bytes;
use futures_util::StreamExt;
use mongodb::bson::oid::ObjectId;
use object_store::path::Path;
use object_store::ObjectStore;
use uuid::Uuid;

use crate::dao::error::RepoError::{Internal, InternalConcrete};
use crate::dao::error::{Entity, RepoResult};

#[derive(Clone)]
pub struct UserRepoRepository {
    store: Arc<dyn ObjectStore>,
}

impl UserRepoRepositoryTrait for UserRepoRepository {}

impl UserRepoRepository {
    pub fn new(store: Arc<dyn ObjectStore>) -> Self {
        Self { store }
    }
}

#[async_trait]
impl BlobConnRepositoryTrait<ObjectId, Uuid> for UserRepoRepository {
    async fn list_pairs(&self, user_id: &ObjectId) -> RepoResult<Vec<Uuid>> {
        let stream = self.store.list(None);

        let mut uuids = vec![];
        for repo in stream.collect::<Vec<_>>().await {
            let repo = repo?;
            let location = &repo.location;
            let filename = location
                .filename()
                .ok_or(InternalConcrete(format!("Wrong location: {location}")))?;
            let split: Vec<_> = filename.split(DELIMITER).collect();
            if split.len() != 2 {
                Err(InternalConcrete(format!("Wrong filename: {filename}")))?
            }

            let curr_user_id = split[0];
            let repo_id = split[1];

            let curr_user_id = ObjectId::from_str(curr_user_id)
                .map_err(|_| InternalConcrete(format!("Wrong user_id: {filename}")))?;

            if curr_user_id == *user_id {
                let uuid = Uuid::from_str(repo_id).map_err(|_| Internal("Invalid uuid"))?;
                let date = repo.last_modified;
                uuids.push((uuid, date));
            }
        }

        uuids.sort_by_key(|t| Reverse(t.1));
        let uuid = uuids.into_iter().map(|t| t.0).collect();
        Ok(uuid)
    }

    async fn add_pair(&self, user_id: &ObjectId, repo_id: &Uuid) -> RepoResult<()> {
        self.is_val_not_connected(repo_id).await?;
        let path = Path::from(self.get_file_name(user_id, repo_id));
        self.store.put(&path, Bytes::new()).await?;
        Ok(())
    }

    async fn delete_pair(&self, user_id: &ObjectId, repo_id: &Uuid) -> RepoResult<()> {
        self.are_connected(user_id, repo_id).await?;
        let path = Path::from(format!("{user_id}{DELIMITER}{repo_id}"));
        self.store.delete(&path).await?;
        Ok(())
    }

    fn store(&self) -> Arc<dyn ObjectStore> {
        Arc::clone(&self.store)
    }

    fn key_entity() -> Entity {
        Entity::User
    }

    fn val_entity() -> Entity {
        Entity::Repository
    }
}
