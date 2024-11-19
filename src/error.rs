use std::io::Error;

use thiserror::Error;

pub type InternalResult<T> = Result<T, InternalError>;

#[derive(Debug, Error)]
pub enum InternalError {
    #[error(transparent)]
    SqlDbConnectionIssue(#[from]  sea_orm::DbErr),

    #[error(transparent)]
    MongoDbConnectionIssue(#[from] mongodb::error::Error),
    
    #[error(transparent)]
    StartServerIssue(#[from] Error),

    #[error(transparent)]
    MessageBroker(#[from] message_broker::error::MBrokerError),

    #[error(transparent)]
    StoreIssue(#[from] object_store::Error),

    #[error("Missing environment variable: {0}")]
    ConfigMissingEnv(&'static str),

    #[error("Can not parse {0}")]
    ConfigParseImpossible(&'static str),
}
