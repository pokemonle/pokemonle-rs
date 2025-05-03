use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Encryption error: {0}")]
    Encryption(String),
    #[error("Decryption error: {0}")]
    Decryption(String),

    #[error(transparent)]
    Base64Decode(#[from] base64::DecodeError),

    #[error("Format error: {0}")]
    Format(String),
}

pub type Result<T> = std::result::Result<T, Error>;
