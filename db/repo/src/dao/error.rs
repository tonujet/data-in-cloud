use mongodb::bson::oid::ObjectId;
use sea_orm::DbErr;
use strum::AsRefStr;
use thiserror::Error;
use uuid::Uuid;

pub type RepoResult<T> = Result<T, RepoError>;

#[derive(AsRefStr, Debug, Clone)]
pub enum Entity {
    Repository,
    User,
    UserRepo,
    UserRepoInfo,
}

#[derive(Error, Debug)]
pub enum RepoError {
    #[error("{} with id {0} not found", .1.as_ref())]
    NotFoundWithUuid(Uuid, Entity),

    #[error("{} with id {0} not found", .1.as_ref())]
    NotFoundWithObjectId(ObjectId, Entity),

    #[error("{} with uuid {0} was deleted", .1.as_ref())]
    DeletedWithUuid(Uuid, Entity),

    #[error("{} with uuid {0} was deleted", .1.as_ref())]
    DeletedWithObjectId(ObjectId, Entity),

    #[error(transparent)]
    SqlExecution(#[from] DbErr),

    #[error("Fields {0:?} of {} already taken by someone", .1.as_ref())]
    Uniqueness(Vec<&'static str>, Entity),

    #[error(transparent)]
    MongoExecution(#[from] mongodb::error::Error),

    #[error("{0}")]
    Internal(&'static str),

    #[error("{0}")]
    InternalConcrete(String),

    #[error("{} already has connection with {}", .0.as_ref(), .1.as_ref())]
    AlreadyConnected(Entity, Entity),

    #[error("{} is not yet connected with {}", .0.as_ref(), .1.as_ref())]
    NotYetConnected(Entity, Entity),

    #[error(transparent)]
    ObjectStore(#[from] object_store::Error),
}
