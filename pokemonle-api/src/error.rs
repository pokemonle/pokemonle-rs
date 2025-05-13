use aide::OperationIo;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, OperationIo)]
pub enum Error {
    #[error("ResourceNotFound")]
    ResourceNotFound(String),

    #[error(transparent)]
    Crypto(#[from] pokemonle_lib::crypto::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        #[derive(Serialize)]
        struct ErrResponse {
            error: String,
        }

        match self {
            Error::ResourceNotFound(s) => {
                let err = ErrResponse { error: s };
                (StatusCode::NOT_FOUND, Json(err)).into_response()
            }
            Error::Crypto(e) => {
                let err = ErrResponse {
                    error: e.to_string(),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(err)).into_response()
            }
        }
    }
}
