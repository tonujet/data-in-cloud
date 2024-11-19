use sea_orm::DbErr;
use strum::AsRefStr;
use thiserror::Error;
use uuid::Uuid;

pub type RepoResult<T> = Result<T, RepoError>;

#[derive(AsRefStr, Debug)]
pub enum Entity {
    Repository
}

#[derive(Error, Debug)]
pub enum RepoError {
    #[error("{} with id {0} not found", .1.as_ref())]
    NotFoundWithUuid(Uuid, Entity),

    #[error("{} with uuid {0} deleted", .1.as_ref())]
    DeletedWithUuid(Uuid, Entity),
    
    #[error(transparent)]
    Execution(#[from] DbErr),
}
