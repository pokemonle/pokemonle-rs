use crate::error::Result;
use aide::axum::{routing::get_with, ApiRouter, IntoApiResponse};
use axum::extract::{Path, Query, State};
use pokemonle_lib::{
    database::{
        entity::{abilities, prelude::*},
        r#trait::LocalizedResourceHandler,
    },
    types::{prelude::*, WithSlot},
};

use super::{
    router::{response_with, ResourceRouter},
    AppState,
};

async fn list_pokemon_with_pagination(
    State(state): State<AppState>,
    Query(PaginateQuery { page, per_page }): Query<PaginateQuery>,
    Query(Language { lang }): Query<Language>,
    Query(SearchQuery { q }): Query<SearchQuery>,
) -> impl IntoApiResponse {
    let handler: &dyn LocalizedResourceHandler<PokemonSpecies, PokemonSpeciesNames> = &state.pool;

    let result = handler.list_with_pagination(page, per_page, lang, q).await;

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

async fn get_pokemon_abilities(
    State(state): State<AppState>,
    Path(ResourceId { id }): Path<ResourceId>,
    Query(Language { lang }): Query<Language>,
    Query(PaginateQuery { page, per_page }): Query<PaginateQuery>,
) -> impl IntoApiResponse {
    let handler = &state.pool;
    Result::from(
        handler
            .get_abilities_by_pokemon_id(id, page, per_page, lang)
            .await,
    )
}

pub fn routers() -> ApiRouter<AppState> {
    let pokemon_species_router = ApiRouter::new()
        .api_route(
            "/",
            get_with(list_pokemon_with_pagination, |op| op.tag("pokemon")),
        )
        .api_route("/{id}", get_with(get_pokemon_by_id, |op| op.tag("pokemon")));

    ApiRouter::new()
        .nest("/pokemon-species", pokemon_species_router)
        .nest(
            "/pokemon",
            Pokemon::routers_with(|op| op.tag("pokemon")).api_route(
                "/{id}/abilities",
                get_with(get_pokemon_abilities, |op| {
                    response_with::<PaginatedResource<WithSlot<WithName<abilities::Model>>>>(op)
                        .tag("pokemon")
                }),
            ),
        )
        .nest(
            "/pokemon-colors",
            PokemonColors::routers_with(|op| op.tag("pokemon")),
        )
        .nest(
            "/pokemon-shapes",
            PokemonShapes::routers_with(|op| op.tag("pokemon")),
        )
        .nest(
            "/pokemon-habitats",
            PokemonHabitats::routers_with(|op| op.tag("pokemon")),
        )
}
