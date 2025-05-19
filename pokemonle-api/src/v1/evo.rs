use crate::error::Result;
use aide::axum::{routing::get_with, ApiRouter, IntoApiResponse};
use axum::extract::{Path, Query, State};
use pokemonle_lib::types::prelude::*;

use crate::v1::router::ResourceRouter;

use super::AppState;

async fn get_pokemon_species_by_evolution_chain_id(
    State(state): State<AppState>,
    Path(ResourceId { id }): Path<ResourceId>,
    Query(Language { lang }): Query<Language>,
) -> impl IntoApiResponse {
    let handler = &state.pool;
    Result::from(
        handler
            .get_pokemon_species_by_evolution_chain_id(id, lang)
            .await,
    )
}

pub fn routers() -> ApiRouter<AppState> {
    use pokemonle_lib::database::entity::prelude::*;

    ApiRouter::new()
        .nest(
            "/evolution-chains",
            EvolutionChains::routers_with(|op| op.tag("evolution")).api_route(
                "/{id}/pokemon-species",
                get_with(get_pokemon_species_by_evolution_chain_id, |op| {
                    op.tag("evolution")
                }),
            ),
        )
        .nest(
            "/evolution-triggers",
            EvolutionTriggers::routers_with(|op| op.tag("evolution")),
        )
}
