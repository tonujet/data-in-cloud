use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::{DateTime, Local};
use futures_util::TryStreamExt;
use mongodb::{bson, Collection, Cursor};
use mongodb::bson::Document;
use mongodb::bson::oid::ObjectId;
use mongodb::options::{AggregateOptions, CountOptions, FindOneOptions, InsertOneOptions, UpdateModifications, UpdateOptions};
use serde::{Deserialize, Serialize};

use crate::MongoCollection;

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
    pub created: DateTime<Local>,
    pub updated: DateTime<Local>,
}

impl User {
    pub fn new(
        email: String,
        username: String,
        password: String,
        age: u8,
        is_public: bool,
        created: DateTime<Local>,
        updated: DateTime<Local>,
    ) -> Self {
        Self {
            id: None,
            deleted: false,
            email,
            username,
            password,
            age,
            is_public,
            created,
            updated,
        }
    }
}


#[derive(Clone)]
pub struct UserCollection {
    pub collection: Collection<User>,
}

#[async_trait]
impl MongoCollection<User> for UserCollection {
    async fn find_one(
        &self,
        filter: Option<Document>,
        options: Option<FindOneOptions>,
    ) -> mongodb::error::Result<Option<User>> {
        self.collection.find_one(filter, options).await
    }

    async fn insert_one(
        &self,
        doc: User,
        options: Option<InsertOneOptions>,
    ) -> mongodb::error::Result<ObjectId> {
        self.collection
            .insert_one(doc, options)
            .await
            .map(|res| res.inserted_id.as_object_id().unwrap())
    }

    async fn update_one(
        &self,
        query: Document,
        update: UpdateModifications,
        options: Option<UpdateOptions>,
    ) -> mongodb::error::Result<()> {
        self.collection
            .update_one(query, update, options)
            .await
            .map(|_d| ())
    }

    async fn aggregate(
        &self,
        pipeline: Vec<Document>,
        options: Option<AggregateOptions>,
    ) -> mongodb::error::Result<Cursor<Document>> {
        self.collection.aggregate(pipeline, options).await
    }

    async fn count_documents(
        &self,
        filter: Option<Document>,
        options: Option<CountOptions>,
    ) -> mongodb::error::Result<u64> {
        self.collection.count_documents(filter, options).await
    }

    async fn aggregate_and_collect(
        &self,
        pipeline: Vec<Document>,
        options: Option<AggregateOptions>,
    ) -> mongodb::error::Result<Vec<User>> {
        Ok(self
            .aggregate(pipeline, options)
            .await?
            .try_collect::<Vec<_>>()
            .await?
            .into_iter()
            .map(|d| bson::from_document::<User>(d).unwrap())
            .collect())
    }
}


pub struct TestUserCollection {
    users: Arc<Mutex<Vec<User>>>,
}

impl TestUserCollection {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(vec![])),
        }
    }
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
        todo!()
    }

    async fn count_documents(
        &self,
        filter: Option<Document>,
        _options: Option<CountOptions>,
    ) -> mongodb::error::Result<u64> {
        let doc: Option<Document> = filter.into();
        if doc == None {
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
        let mut skip: Option<usize> = None;
        let mut limit: Option<usize> = None;

        for doc in pipeline {
            let poss_skip = doc.get("$skip");
            let poss_limit = doc.get("$limit");

            if let (None, Some(poss_skip)) = (skip, poss_skip) {
                skip = Some(poss_skip.as_i32().unwrap() as usize)
            }

            if let (None, Some(poss_limit)) = (skip, poss_limit) {
                limit = Some(poss_limit.as_i32().unwrap() as usize)
            }
        }

        let users: Vec<User> = users.into_iter().filter(|u| u.deleted == false).collect();

        let skip = skip.unwrap();
        let limit = limit.unwrap_or(users.len());

        let users = users.into_iter().skip(skip).take(limit).collect();
        Ok(users)
    }
}

