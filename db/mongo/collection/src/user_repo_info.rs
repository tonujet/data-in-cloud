use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use bson::serde_helpers::uuid_1_as_binary;
use chrono::{DateTime, Local, Utc};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::Document;
use mongodb::options::{
    AggregateOptions, CountOptions, FindOneOptions, InsertOneOptions, UpdateModifications,
    UpdateOptions,
};
use mongodb::{Collection, Cursor};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{utils, MongoCollection};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRepoInfo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,

    #[serde(with = "uuid_1_as_binary")]
    pub repo_id: Uuid,
    pub operation: UserRepoInfoOperation,
    pub executed_at: DateTime<Utc>,
}

impl UserRepoInfo {
    pub fn new(user_id: ObjectId, repo_id: Uuid, operation: UserRepoInfoOperation) -> UserRepoInfo {
        let executed_at = Utc::now();
        Self {
            id: None,
            user_id,
            repo_id,
            operation,
            executed_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy, Eq)]
#[derive(async_graphql::Enum)]
pub enum UserRepoInfoOperation {
    CreateLink,
    DeleteLink,
}

pub struct UserRepoInfoCollection {
    pub collection: Collection<UserRepoInfo>,
}

impl MongoCollection<UserRepoInfo> for UserRepoInfoCollection {
    fn get_collection(&self) -> Option<&Collection<UserRepoInfo>> {
        Some(&self.collection)
    }
}

pub struct TestUserRepoInfoCollection {
    entities: Arc<Mutex<Vec<UserRepoInfo>>>,
}

impl TestUserRepoInfoCollection {
    pub fn new() -> Self {
        Self {
            entities: Arc::new(Mutex::new(vec![])),
        }
    }
}

#[async_trait]
impl MongoCollection<UserRepoInfo> for TestUserRepoInfoCollection {
    async fn find_one(
        &self,
        filter: Option<Document>,
        _options: Option<FindOneOptions>,
    ) -> mongodb::error::Result<Option<UserRepoInfo>> {
        let filter = filter.unwrap();
        let id = filter.get("_id");
        let entities = self.entities.lock().unwrap();
        if let Some(id) = id {
            return Ok(entities
                .iter()
                .find(|&u| u.id == id.as_object_id())
                .cloned());
        }
        panic!("Not yet implemented logic")
    }

    async fn insert_one(
        &self,
        mut doc: UserRepoInfo,
        _options: Option<InsertOneOptions>,
    ) -> mongodb::error::Result<ObjectId> {
        let id = ObjectId::new();
        doc.id = Some(id);
        self.entities.lock().unwrap().push(doc);
        Ok(id)
    }

    async fn update_one(
        &self,
        _query: Document,
        _update: UpdateModifications,
        _options: Option<UpdateOptions>,
    ) -> mongodb::error::Result<()> {
        panic!("This test collection can't be updated")
    }

    async fn aggregate(
        &self,
        _pipeline: Vec<Document>,
        _options: Option<AggregateOptions>,
    ) -> mongodb::error::Result<Cursor<Document>> {
        panic!("This test collection can't be aggregated")
    }

    async fn count_documents(
        &self,
        _filter: Option<Document>,
        _options: Option<CountOptions>,
    ) -> mongodb::error::Result<u64> {
        let entities = self.entities.lock().unwrap();
        Ok(entities.len() as u64)
    }

    async fn aggregate_and_collect(
        &self,
        pipeline: Vec<Document>,
        _options: Option<AggregateOptions>,
    ) -> mongodb::error::Result<Vec<UserRepoInfo>> {
        let mut entities = self.entities.lock().unwrap().clone();
        let r#match = pipeline
            .first()
            .and_then(|val| val.get("$match").and_then(|r#match| r#match.as_document()));
        if let Some(doc) = r#match {
            println!("{doc:?}");
            match doc.get("user_id") {
                Some(user_id) => {
                    println!("{user_id:?}");
                    entities.retain(|u| u.user_id == user_id.as_object_id().unwrap());
                }
                None => panic!("user_id match parameters not implemented"),
            }
        };
        Ok(utils::paginate_inmemory_collection(entities, pipeline))
    }
}
