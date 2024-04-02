use async_trait::async_trait;
use mongodb::bson::Document;
use mongodb::bson::oid::ObjectId;
use mongodb::Cursor;
use mongodb::options::{AggregateOptions, CountOptions, FindOneOptions, InsertOneOptions, UpdateModifications, UpdateOptions};

pub mod user;


#[async_trait]
pub trait MongoCollection<T>: Send + Sync {
    async fn find_one(
        &self,
        filter: Option<Document>,
        options: Option<FindOneOptions>,
    ) -> mongodb::error::Result<Option<T>>;

    async fn insert_one(
        &self,
        doc: T,
        options: Option<InsertOneOptions>,
    ) -> mongodb::error::Result<ObjectId>;

    async fn update_one(
        &self,
        query: Document,
        update: UpdateModifications,
        options: Option<UpdateOptions>,
    ) -> mongodb::error::Result<()>;

    async fn aggregate(
        &self,
        pipeline: Vec<Document>,
        options: Option<AggregateOptions>,
    ) -> mongodb::error::Result<Cursor<Document>>;

    async fn count_documents(
        &self,
        filter: Option<Document>,
        options: Option<CountOptions>,
    ) -> mongodb::error::Result<u64>;

    async fn aggregate_and_collect(
        &self,
        pipeline: Vec<Document>,
        options: Option<AggregateOptions>,
    ) -> mongodb::error::Result<Vec<T>>;
}
