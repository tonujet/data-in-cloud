use async_trait::async_trait;
use futures_util::TryStreamExt;
use mongodb::bson::Document;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson, Collection, Cursor};
use mongodb::options::{AggregateOptions, CountOptions, FindOneOptions, InsertOneOptions, UpdateModifications, UpdateOptions};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod user;
pub mod user_repo_info;


#[async_trait]
pub trait MongoCollection<T: Serialize + DeserializeOwned + Unpin + Send + Sync>: Send + Sync {
    fn get_collection(&self) -> Option<&Collection<T>> {
        None
    }
    
    async fn find_one(
        &self,
        filter: Option<Document>,
        options: Option<FindOneOptions>,
    ) -> mongodb::error::Result<Option<T>>{
        self.get_collection().unwrap().find_one(filter, options).await
    }

    async fn insert_one(
        &self,
        doc: T,
        options: Option<InsertOneOptions>,
    ) -> mongodb::error::Result<ObjectId> where T: 'async_trait {
        self.get_collection().unwrap()
            .insert_one(doc, options)
            .await
            .map(|res| res.inserted_id.as_object_id().unwrap())
    }

    async fn update_one(
        &self,
        query: Document,
        update: UpdateModifications,
        options: Option<UpdateOptions>,
    ) -> mongodb::error::Result<()>{
        self.get_collection().unwrap()
            .update_one(query, update, options)
            .await
            .map(|_d| ())
    }

    async fn aggregate(
        &self,
        pipeline: Vec<Document>,
        options: Option<AggregateOptions>,
    ) -> mongodb::error::Result<Cursor<Document>> {
        self.get_collection().unwrap().aggregate(pipeline, options).await
    }

    async fn count_documents(
        &self,
        filter: Option<Document>,
        options: Option<CountOptions>,
    ) -> mongodb::error::Result<u64> {
        self.get_collection().unwrap().count_documents(filter, options).await
    }

    async fn aggregate_and_collect(
        &self,
        pipeline: Vec<Document>,
        options: Option<AggregateOptions>,
    ) -> mongodb::error::Result<Vec<T>> {
        Ok(self
            .aggregate(pipeline, options)
            .await?
            .try_collect::<Vec<_>>()
            .await?
            .into_iter()
            .map(|d| bson::from_document::<T>(d).unwrap())
            .collect())
    }
}
