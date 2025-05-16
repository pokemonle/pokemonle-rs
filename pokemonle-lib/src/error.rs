use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("SampleError")]
    SampleError,

    #[error("Env var '{0}' does not exist; {1}")]
    ConfigReadNonExistEnvVar(&'static str, #[source] std::env::VarError),

    #[error("Env var '{0}' is empty")]
    ConfigReadEmptyEnvVar(&'static str),

    #[error(transparent)]
    ConnectionError(#[from] sea_orm::DbErr),

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Unsupported database url '{0}'")]
    UnsupportedDatabase(String),

    #[error("Mutex lock was poisoned")]
    MutexPoisonError,

    #[error(transparent)]
    SerdePlainError(#[from] serde_plain::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
