use std::io::Error;

use sea_orm::DbErr;
use thiserror::Error;

pub type InternalResult<T> = Result<T, InternalError>;

#[derive(Debug, Error)]
pub enum InternalError {
    #[error(transparent)]
    DbConnectionIssue(#[from] DbErr),

    #[error(transparent)]
    StartServerIssue(#[from] Error),

    #[error("{0}")]
    ConfigMissingEnv(&'static str),

    #[error("{0}")]
    ConfigParseImpossible(&'static str),
}
