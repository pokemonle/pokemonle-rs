use aide::OperationIo;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, OperationIo)]
pub enum Error {
    #[error(transparent)]
    PokemonleLib(#[from] pokemonle_lib::error::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        #[derive(Serialize)]
        struct ErrResponse {
            error: String,
        }

        match self {
            Error::PokemonleLib(e) => {
                let err = ErrResponse {
                    error: e.to_string(),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(err)).into_response()
            }
        }
    }
}

#[derive(Debug, OperationIo)]
pub struct Result<T>(pub std::result::Result<T, Error>);

impl<T> IntoResponse for Result<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        match self.0 {
            Ok(data) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => e.into_response(),
        }
    }
}

impl<T> From<pokemonle_lib::error::Result<T>> for Result<T> {
    fn from(value: pokemonle_lib::error::Result<T>) -> Self {
        match value {
            Ok(data) => Result(Ok(data)),
            Err(e) => Result(Err(Error::PokemonleLib(e))),
        }
    }
}

impl<T> From<pokemonle_lib::error::Error> for Result<T> {
    fn from(value: pokemonle_lib::error::Error) -> Self {
        Result(Err(Error::PokemonleLib(value)))
    }
}
