use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use aide::axum::{routing::get_with, ApiRouter, IntoApiResponse};
use pokemonle_lib::model::Version;

use super::{
    response::{get_item_by_id_docs, list_items_docs},
    AppState, ListResponse,
};

async fn list_version(State(state): State<AppState>) -> impl IntoApiResponse {
    let versions = state.pool.version_handler().get_all_versions();

    let total = versions.len();

    (
        StatusCode::OK,
        Json(ListResponse {
            data: versions,
            total,
        }),
    )
}

async fn get_version_by_id(
    State(state): State<AppState>,
    Path(version_id): Path<i32>,
) -> impl IntoApiResponse {
    let version = state.pool.version_handler().get_version_by_id(version_id);

    match version {
        Some(l) => Json(l).into_response(),
        None => crate::error::Error::ResourceNotFound(format!(
            "Version with id {} not found",
            version_id
        ))
        .into_response(),
    }
}

pub fn routers() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route("/", get_with(list_version, list_items_docs::<Version>))
        .api_route(
            "/{version_id}",
            get_with(get_version_by_id, get_item_by_id_docs::<Version>),
        )
}
