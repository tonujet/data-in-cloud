use async_trait::async_trait;
use bson::doc;
use futures_util::TryStreamExt;
use mongodb::{bson, Collection, Cursor};
use mongodb::bson::Document;
use mongodb::bson::oid::ObjectId;
use mongodb::options::{
    AggregateOptions, CountOptions, FindOneOptions, InsertOneOptions, UpdateModifications,
    UpdateOptions,
};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod user;
pub mod user_repo_info;

#[async_trait]
pub trait MongoCollection<T: Serialize + DeserializeOwned + Unpin + Send + Sync>:
    Send + Sync
{
    fn get_collection(&self) -> Option<&Collection<T>> {
        None
    }

    async fn find_one(
        &self,
        filter: Option<Document>,
        options: Option<FindOneOptions>,
    ) -> mongodb::error::Result<Option<T>> {
        self.get_collection()
            .unwrap()
            .find_one(filter, options)
            .await
    }

    async fn insert_one(
        &self,
        doc: T,
        options: Option<InsertOneOptions>,
    ) -> mongodb::error::Result<ObjectId>
    where
        T: 'async_trait,
    {
        self.get_collection()
            .unwrap()
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
        self.get_collection()
            .unwrap()
            .update_one(query, update, options)
            .await
            .map(|_d| ())
    }

    async fn aggregate(
        &self,
        pipeline: Vec<Document>,
        options: Option<AggregateOptions>,
    ) -> mongodb::error::Result<Cursor<Document>> {
        self.get_collection()
            .unwrap()
            .aggregate(pipeline, options)
            .await
    }

    async fn count_documents(
        &self,
        filter: Option<Document>,
        options: Option<CountOptions>,
    ) -> mongodb::error::Result<u64> {
        self.get_collection()
            .unwrap()
            .count_documents(filter, options)
            .await
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

    async fn paginate_pipeline_and_collect(
        &self,
        mut pipeline: Vec<Document>,
        take: Option<u64>,
        offset: Option<u64>,
        options: Option<AggregateOptions>,
    ) -> mongodb::error::Result<Vec<T>> {
        let skip = doc! {"$skip" : offset.unwrap_or(0) as u32};
        pipeline.push(skip);

        if let Some(take) = take.filter(|&take| take != 0) {
            pipeline.push(doc! {"$limit" : take as u32})
        }

        Ok(self.aggregate_and_collect(pipeline, options).await?)
    }
}
