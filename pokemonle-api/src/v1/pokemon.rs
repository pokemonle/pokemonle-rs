use crate::error::Result;
use aide::axum::{routing::get_with, ApiRouter, IntoApiResponse};
use axum::extract::{Path, Query, State};
use pokemonle_lib::{
    database::{entity::prelude::*, r#trait::LocalizedResourceHandler},
    types::request::{Language, PaginateQuery, ResourceId},
};

use super::AppState;

async fn list_pokemon_with_pagination(
    State(state): State<AppState>,
    Query(PaginateQuery { page, per_page }): Query<PaginateQuery>,
    Query(Language { lang }): Query<Language>,
) -> impl IntoApiResponse {
    let handler: &dyn LocalizedResourceHandler<PokemonSpecies, PokemonSpeciesNames> = &state.pool;

    let result = handler.list_with_pagination(page, per_page, lang).await;

    Result::from(result)
}

async fn get_pokemon_by_id(
    State(state): State<AppState>,
    Path(ResourceId { id }): Path<ResourceId>,
    Query(Language { lang }): Query<Language>,
) -> impl IntoApiResponse {
    let handler: &dyn LocalizedResourceHandler<PokemonSpecies, PokemonSpeciesNames> = &state.pool;
    Result::from(handler.get_by_id(id, lang).await)
}

pub fn routers() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list_pokemon_with_pagination, |op| op.tag("pokemon")),
        )
        .api_route("/{id}", get_with(get_pokemon_by_id, |op| op.tag("pokemon")))
}
