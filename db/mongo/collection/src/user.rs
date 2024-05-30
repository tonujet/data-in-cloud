use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::{DateTime, Local, Utc};
use mongodb::{Collection, Cursor};
use mongodb::bson::Document;
use mongodb::bson::oid::ObjectId;
use mongodb::options::{
    AggregateOptions, CountOptions, FindOneOptions, InsertOneOptions, UpdateModifications,
    UpdateOptions,
};
use serde::{Deserialize, Serialize};

use crate::{MongoCollection, utils};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub username: String,
    pub password: String,
    pub age: u8,
    pub is_public: bool,
    pub deleted: bool,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl User {
    pub fn new(
        email: String,
        username: String,
        password: String,
        age: u8,
        is_public: bool,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            deleted: false,
            email,
            username,
            password,
            age,
            is_public,
            created: now,
            updated: now,
        }
    }
}

#[derive(Clone)]
pub struct UserCollection {
    collection: Collection<User>,
}

impl UserCollection {
    pub fn new(collection: Collection<User>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl MongoCollection<User> for UserCollection {
    fn get_collection(&self) -> Option<&Collection<User>> {
        Some(&self.collection)
    }
}

#[derive(Default)]
pub struct TestUserCollection {
    users: Arc<Mutex<Vec<User>>>,
}
#[async_trait]
impl MongoCollection<User> for TestUserCollection {
    async fn find_one(
        &self,
        filter: Option<Document>,
        _options: Option<FindOneOptions>,
    ) -> mongodb::error::Result<Option<User>> {
        let filter = filter.unwrap();
        let id = filter.get("_id");
        let username = filter.get("username");
        let email = filter.get("email");

        let users = self.users.lock().unwrap();

        if let Some(id) = id {
            return Ok(users.iter().find(|&u| u.id == id.as_object_id()).cloned());
        }

        if let Some(username) = username {
            return Ok(users
                .iter()
                .find(|&u| u.username == username.as_str().unwrap())
                .cloned());
        }

        if let Some(email) = email {
            return Ok(users
                .iter()
                .find(|&u| u.email == email.as_str().unwrap())
                .cloned());
        }

        Ok(None)
    }

    async fn insert_one(
        &self,
        mut doc: User,
        _options: Option<InsertOneOptions>,
    ) -> mongodb::error::Result<ObjectId> {
        let id = ObjectId::new();
        doc.id = Some(id);
        self.users.lock().unwrap().push(doc);
        Ok(id)
    }

    async fn update_one(
        &self,
        query: Document,
        update: UpdateModifications,
        _options: Option<UpdateOptions>,
    ) -> mongodb::error::Result<()> {
        let id = query.get("_id").unwrap().as_object_id().unwrap();
        let mut users = self.users.lock().unwrap();
        let user = users
            .iter_mut()
            .find(|u| u.id.unwrap() == id)
            .expect("User not found");
        let doc = match update {
            UpdateModifications::Document(doc) => doc,
            _ => panic!("Not implemented yet"),
        };

        let doc = doc.get("$set").unwrap().as_document().unwrap();
        let deleted = doc.get("deleted");
        let username = doc.get("username");
        let age = doc.get("age");
        let is_public = doc.get("is_public");
        let updated = doc.get("updated");

        if let Some(deleted) = deleted {
            user.deleted = deleted.as_bool().unwrap();
            return Ok(());
        }

        user.username = username.unwrap().as_str().unwrap().to_string();
        user.age = age.unwrap().as_i32().unwrap() as u8;
        user.is_public = is_public.unwrap().as_bool().unwrap();
        user.updated = updated
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
            .parse()
            .unwrap();
        Ok(())
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
        filter: Option<Document>,
        _options: Option<CountOptions>,
    ) -> mongodb::error::Result<u64> {
        let doc: Option<Document> = filter;
        if doc.is_none() {
            return Ok(self.users.lock().unwrap().len() as u64);
        }

        let doc = doc.unwrap();

        let deleted = doc.get("deleted");

        if let Some(deleted) = deleted {
            return Ok(self
                .users
                .lock()
                .unwrap()
                .iter()
                .filter(|d| d.deleted == deleted.as_bool().unwrap())
                .count() as u64);
        }

        panic!("Other options not implemented yet")
    }

    async fn aggregate_and_collect(
        &self,
        pipeline: Vec<Document>,
        _options: Option<AggregateOptions>,
    ) -> mongodb::error::Result<Vec<User>> {
        let users = self.users.lock().unwrap().clone();
        let users = users.into_iter().filter(|u| !u.deleted).collect();
        Ok(utils::paginate_inmemory_collection(users, pipeline))
    }
}
