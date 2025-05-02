use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use aide::axum::{routing::get_with, ApiRouter, IntoApiResponse};
use pokemonle_lib::model::Ability;

use super::{
    response::{get_item_by_id_docs, list_items_docs},
    AppState, ListResponse,
};

async fn list_abilities(State(state): State<AppState>) -> impl IntoApiResponse {
    let abilities = state.pool.ability_handler().get_all_abilities();

    let total = abilities.len();

    (
        StatusCode::OK,
        Json(ListResponse {
            data: abilities,
            total,
        }),
    )
}

async fn get_ability_by_id(
    State(state): State<AppState>,
    Path(ability_id): Path<i32>,
) -> impl IntoApiResponse {
    let ability = state.pool.ability_handler().get_ability_by_id(ability_id);

    match ability {
        Some(l) => Json(l).into_response(),
        None => crate::error::Error::ResourceNotFound(format!(
            "Ability with id {} not found",
            ability_id
        ))
        .into_response(),
    }
}

pub fn routers() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route("/", get_with(list_abilities, list_items_docs::<Ability>))
        .api_route(
            "/{ability_id}",
            get_with(get_ability_by_id, get_item_by_id_docs::<Ability>),
        )
}
