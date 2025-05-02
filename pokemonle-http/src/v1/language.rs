use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use aide::axum::{routing::get_with, ApiRouter, IntoApiResponse};
use pokemonle_lib::model::Language;

use super::{
    response::{get_item_by_id_docs, list_items_docs},
    AppState, ListResponse,
};

async fn list_language(State(state): State<AppState>) -> impl IntoApiResponse {
    let languages = state.pool.language_handler().get_all_languages();

    let total = languages.len();

    (
        StatusCode::OK,
        Json(ListResponse {
            data: languages,
            total,
        }),
    )
}

async fn get_language_by_id(
    State(state): State<AppState>,
    Path(language_id): Path<i32>,
) -> impl IntoApiResponse {
    let language = state
        .pool
        .language_handler()
        .get_language_by_id(language_id);

    match language {
        Some(l) => Json(l).into_response(),
        None => crate::error::Error::ResourceNotFound(format!(
            "Language with id {} not found",
            language_id
        ))
        .into_response(),
    }
}

pub fn routers() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route("/", get_with(list_language, list_items_docs::<Language>))
        .api_route(
            "/{language_id}",
            get_with(get_language_by_id, get_item_by_id_docs::<Language>),
        )
}
