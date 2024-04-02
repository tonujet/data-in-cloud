use mongodb::options::{CreateCollectionOptions, CreateIndexOptions};
use mongodb::{Collection, Database, IndexModel};
use collection::user::User;

use crate::error::SchemeResult;
use crate::user::UserScheme;

pub mod error;
pub mod user;


trait GetScheme<Entity = Self> {
    fn get_scheme() -> impl Scheme<Entity = Entity>;
}

impl GetScheme for User {
    fn get_scheme() -> impl Scheme<Entity = Self> {
        UserScheme {}
    }
}

pub async fn get_collection<T: GetScheme>(database: &Database) -> SchemeResult<Collection<T>> {
    T::get_scheme().get_collection(database).await
}

// Template method pattern implementation
#[warn(async_fn_in_trait)]
pub trait Scheme {
    type Entity;
    fn get_collection_name(&self) -> &'static str;
    fn get_validation_options(&self) -> CreateCollectionOptions;
    fn get_indexes(&self) -> Vec<(IndexModel, impl Into<Option<CreateIndexOptions>>)>;

    #[warn(async_fn_in_trait)]
    async fn get_collection(&self, db: &Database) -> SchemeResult<Collection<Self::Entity>> {
        let name = self.get_collection_name();
        let ops = self.get_validation_options();
        db.create_collection(name, ops).await?;
        let collection: Collection<Self::Entity> = db.collection(name);
        let indexes = self.get_indexes();
        for (index, option) in indexes {
            collection.create_index(index, option).await?;
        }
        Ok(collection)
    }

    fn new() -> Self;
}
