use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("ResourceNotFound")]
    ResourceNotFound(String),
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
        }
    }
}
