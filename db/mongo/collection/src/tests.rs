use async_trait::async_trait;
use bson::Document;
use bson::oid::ObjectId;
use mongodb::options::{AggregateOptions, InsertOneOptions};
use tokio::sync::Mutex;

use crate::{MongoCollection, utils};

#[derive(Default)]
struct TestCollection {
    numbers: Mutex<Vec<u8>>,
}
#[async_trait]
impl MongoCollection<u8> for TestCollection {
    async fn insert_one(
        &self,
        doc: u8,
        _options: Option<InsertOneOptions>,
    ) -> mongodb::error::Result<ObjectId>
    where
        u8: 'async_trait,
    {
        self.numbers.lock().await.push(doc);
        Ok(ObjectId::new())
    }

    async fn aggregate_and_collect(
        &self,
        pipeline: Vec<Document>,
        _options: Option<AggregateOptions>,
    ) -> mongodb::error::Result<Vec<u8>> {
        let users = self.numbers.lock().await.clone();
        Ok(utils::paginate_inmemory_collection(users, pipeline))
    }
}

#[tokio::test]
async fn paginate_pipeline_and_collect_with_skip_success() {
    let collection = TestCollection::default();
    let numbers: Vec<u8> = (0..10).collect();
    let desired_numbers: Vec<u8> = (5..10).collect();
    let skip = Some(5);
    let take = None;
    let pipeline = vec![];

    for number in numbers {
        collection.insert_one(number, None).await.unwrap();
    }
    let paginated_numbers = collection
        .paginate_pipeline_and_collect(pipeline, take, skip, None)
        .await
        .unwrap();

    assert_eq!(paginated_numbers, desired_numbers);
}

#[tokio::test]
async fn paginate_pipeline_and_collect_without_skip_and_take_success() {
    let collection = TestCollection::default();
    let numbers: Vec<u8> = (0..10).collect();
    let desired_numbers: Vec<u8> = (0..10).collect();
    let skip = None;
    let take = None;
    let pipeline = vec![];

    for number in numbers {
        collection.insert_one(number, None).await.unwrap();
    }
    let paginated_numbers = collection
        .paginate_pipeline_and_collect(pipeline, take, skip, None)
        .await
        .unwrap();

    assert_eq!(paginated_numbers, desired_numbers);
}

#[tokio::test]
async fn paginate_pipeline_and_collect_with_take_success() {
    let collection = TestCollection::default();
    let numbers: Vec<u8> = (0..10).collect();
    let desired_numbers: Vec<u8> = (0..5).collect();
    let skip = None;
    let take = Some(5);
    let pipeline = vec![];

    for number in numbers {
        collection.insert_one(number, None).await.unwrap();
    }
    let paginated_numbers = collection
        .paginate_pipeline_and_collect(pipeline, take, skip, None)
        .await
        .unwrap();

    assert_eq!(paginated_numbers, desired_numbers);
}
