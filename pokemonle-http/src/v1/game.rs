use crate::error::Error;
use aide::{
    axum::{routing::get_with, ApiRouter, IntoApiResponse},
    transform::TransformOperation,
};
use async_session::serde_json::{self};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use pokemonle_lib::crypto::Error as CryptoError;
use pokemonle_lib::model::PokemonSpecies;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::AppState;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
struct InitGameQuery {
    #[validate(range(min = 0, max = 511))]
    #[serde(default = "default_encode")]
    pub encode: u16,
}

#[inline]
fn default_encode() -> u16 {
    511
}

impl InitGameQuery {
    pub fn indices(self) -> Vec<usize> {
        (0..16)
            .filter(|&bit| (self.encode & (1 << bit)) != 0)
            .map(|bit| bit + 1)
            .collect()
    }
}

async fn init_game(
    State(state): State<AppState>,
    Query(q): Query<InitGameQuery>,
) -> impl IntoApiResponse {
    match state
        .pool
        .pokemon_handler()
        .get_random_pokemon(&q.indices())
        .ok_or_else(|| return Error::ResourceNotFound(String::from("No pokemon found")))
        .and_then(|pm| {
            serde_json::to_vec(&pm)
                .map_err(CryptoError::SerdeJsonError)
                .and_then(|data| state.crypto.encrypt(&data))
                .map_err(Error::Crypto)
        }) {
        Ok(encrypted) => (StatusCode::OK, String::from_utf8(encrypted).unwrap()).into_response(),
        Err(e) => e.into_response(),
    }
}

fn init_game_docs(op: TransformOperation) -> TransformOperation {
    op.tag("game")
        .response_with::<200, &str, _>(|res| res.description("example").example("encrypted data"))
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
struct StopGameQuery {
    pub data: String,
}

async fn stop_game(
    State(state): State<AppState>,
    Query(StopGameQuery { data }): Query<StopGameQuery>,
) -> impl IntoApiResponse {
    match state
        .crypto
        .decrypt(data.trim().replace("\"", "").as_bytes())
        .and_then(|decrypted| {
            serde_json::from_slice::<PokemonSpecies>(&decrypted)
                .map_err(CryptoError::SerdeJsonError)
        })
        .map_err(Error::Crypto)
    {
        Ok(pokemon) => (StatusCode::OK, Json(pokemon)).into_response(),
        Err(e) => e.into_response(),
    }
}

fn stop_game_docs(op: TransformOperation) -> TransformOperation {
    op.tag("game")
        .description("Stop the game")
        .response_with::<200, Json<PokemonSpecies>, _>(|res| res.description("example"))
        .response_with::<400, (), _>(|res| res.description("Invalid data"))
}

pub fn routers() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route("/init", get_with(init_game, init_game_docs))
        .api_route("/giveup", get_with(stop_game, stop_game_docs))
}
