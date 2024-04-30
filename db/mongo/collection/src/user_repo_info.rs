use std::sync::{Arc, Mutex};

use chrono::{DateTime, Local};
use mongodb::bson::oid::ObjectId;
use mongodb::{Collection, Cursor};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::MongoCollection;

use async_trait::async_trait;
use mongodb::bson::Document;
use mongodb::options::{AggregateOptions, CountOptions, FindOneOptions, InsertOneOptions, UpdateModifications, UpdateOptions};
use bson::serde_helpers::uuid_1_as_binary;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRepoInfo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,

    #[serde(with = "uuid_1_as_binary")]
    pub repo_id: Uuid,
    pub operation: UserRepoInfoOperation,
    pub executed_at: DateTime<Local>,
}

impl UserRepoInfo {
    pub fn new(user_id: ObjectId, repo_id: Uuid, operation: UserRepoInfoOperation) -> UserRepoInfo {
        let executed_at = Local::now();
        Self {
            id: None,
            user_id,
            repo_id,
            operation,
            executed_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    infos: Arc<Mutex<Vec<UserRepoInfo>>>,
}

impl TestUserRepoInfoCollection {
    pub fn new() -> Self {
        Self {
            infos: Arc::new(Mutex::new(vec![])),
        }
    }
}


#[async_trait]
impl MongoCollection<UserRepoInfo> for TestUserRepoInfoCollection {
    async fn find_one(&self, filter: Option<Document>, options: Option<FindOneOptions>) -> mongodb::error::Result<Option<UserRepoInfo>> {
        todo!()
    }

    async fn insert_one(&self, doc: UserRepoInfo, options: Option<InsertOneOptions>) -> mongodb::error::Result<ObjectId> {
        todo!()
    }

    async fn update_one(&self, query: Document, update: UpdateModifications, options: Option<UpdateOptions>) -> mongodb::error::Result<()> {
        todo!()
    }

    async fn aggregate(&self, pipeline: Vec<Document>, options: Option<AggregateOptions>) -> mongodb::error::Result<Cursor<Document>> {
        todo!()
    }

    async fn count_documents(&self, filter: Option<Document>, options: Option<CountOptions>) -> mongodb::error::Result<u64> {
        todo!()
    }

    async fn aggregate_and_collect(&self, pipeline: Vec<Document>, options: Option<AggregateOptions>) -> mongodb::error::Result<Vec<UserRepoInfo>> {
        todo!()
    }
}
