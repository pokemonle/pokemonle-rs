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
    ConnectionError(#[from] diesel::ConnectionError),

    #[error("Unsupported database url '{0}'")]
    UnsupportedDatabase(String),

    #[error(transparent)]
    DieselError(#[from] diesel::result::Error),

    #[error(transparent)]
    R2D2PoolError(#[from] diesel::r2d2::PoolError),

    #[error(transparent)]
    R2D2Error(#[from] diesel::r2d2::Error),

    #[error("Mutex lock was poisoned")]
    MutexPoisonError,
}

pub type Result<T> = std::result::Result<T, Error>;
