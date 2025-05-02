use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use aide::axum::{routing::get_with, ApiRouter, IntoApiResponse};
use pokemonle_lib::model::VersionGroup;

use super::{
    response::{get_item_by_id_docs, list_items_docs},
    AppState, ListResponse,
};

async fn list_version_group(State(state): State<AppState>) -> impl IntoApiResponse {
    let version_groups = state.pool.version_group_handler().get_all_version_groups();

    let total = version_groups.len();

    (
        StatusCode::OK,
        Json(ListResponse {
            data: version_groups,
            total,
        }),
    )
}

async fn get_version_group_by_id(
    State(state): State<AppState>,
    Path(version_group_id): Path<i32>,
) -> impl IntoApiResponse {
    let version_group = state
        .pool
        .version_group_handler()
        .get_version_group_by_id(version_group_id);

    match version_group {
        Some(l) => Json(l).into_response(),
        None => crate::error::Error::ResourceNotFound(format!(
            "VersionGroup with id {} not found",
            version_group_id
        ))
        .into_response(),
    }
}

pub fn routers() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list_version_group, list_items_docs::<VersionGroup>),
        )
        .api_route(
            "/{version_group_id}",
            get_with(get_version_group_by_id, get_item_by_id_docs::<VersionGroup>),
        )
}
