mod ability;
mod language;
mod openapi;
mod pokemon;
mod resource;
mod response;
mod router;

use crate::error::Result;
use router::{api_flavor_text_routers_with_transform, Language};

use aide::axum::{routing::get_with, ApiRouter, IntoApiResponse};

use axum::{
    extract::{Path, Query, State},
    Json,
};
use pokemonle_lib::{
    database::{handler::DatabaseClientPooled, pagination::PaginatedResource},
    model::{Generation, Languaged, Pokemon, PokemonSpecies, Type},
};
use router::{api_languaged_routers, api_routers};
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Clone)]
pub struct AppState {
    pub pool: DatabaseClientPooled,
}

#[derive(Deserialize, JsonSchema)]
pub struct Resource {
    id: i32,
}

#[derive(Deserialize, JsonSchema)]
pub struct VersionGroup {
    version_group: i32,
}

// #[derive(Deserialize, JsonSchema)]
// pub struct Version {
//     id: i32,
// }

fn item_routers() -> ApiRouter<AppState> {
    use pokemonle_lib::model::{Item, ItemCategory, ItemPocket};
    ApiRouter::new()
        .nest(
            "/items",
            api_languaged_routers::<Item, _, _>(|state| state.pool.item()).nest(
                "/{id}/flavor-text",
                api_flavor_text_routers_with_transform::<Item, _, _, _>(
                    |state| state.pool.item(),
                    |op| op.tag("item"),
                ),
            ),
        )
        .nest(
            "/item-categories",
            api_routers::<ItemCategory, _, _>(|state| state.pool.item_category()),
        )
        .nest(
            "/item-pockets",
            api_languaged_routers::<ItemPocket, _, _>(|state| state.pool.item_pocket()),
        )
}

fn berry_routers() -> ApiRouter<AppState> {
    use pokemonle_lib::model::{Berry, BerryFirmness};
    ApiRouter::new()
        .nest(
            "/berries",
            api_routers::<Berry, _, _>(|state| state.pool.berry()),
        )
        .nest(
            "/berry-firmness",
            api_routers::<BerryFirmness, _, _>(|state| state.pool.berry_firmness()),
        )
}

fn contest_routers() -> ApiRouter<AppState> {
    use pokemonle_lib::model::{ContestEffect, ContestType};
    ApiRouter::new()
        .nest(
            "/contest-effects",
            api_routers::<ContestEffect, _, _>(|state| state.pool.contest_effect()),
        )
        .nest(
            "/contest-types",
            api_routers::<ContestType, _, _>(|state| state.pool.contest_type()),
        )
}

fn encounter_routers() -> ApiRouter<AppState> {
    use pokemonle_lib::model::{
        Encounter, EncounterCondition, EncounterConditionValue, EncounterSlot,
    };
    ApiRouter::new()
        .nest(
            "/encounters",
            api_routers::<Encounter, _, _>(|state| state.pool.encounter()),
        )
        .nest(
            "/encounter-conditions",
            api_routers::<EncounterCondition, _, _>(|state| state.pool.encounter_condition()),
        )
        .nest(
            "/encounter-condition-values",
            api_routers::<EncounterConditionValue, _, _>(|state| {
                state.pool.encounter_condition_value()
            }),
        )
        .nest(
            "/encounter-slots",
            api_routers::<EncounterSlot, _, _>(|state| state.pool.encounter_slot()),
        )
}

fn evolution_routers() -> ApiRouter<AppState> {
    use pokemonle_lib::model::{EvolutionChain, EvolutionTrigger};

    async fn get_evolution_chain_pokemon_species(
        State(state): State<AppState>,
        Path(Resource { id }): Path<Resource>,
        Query(Language { lang }): Query<Language>,
    ) -> impl IntoApiResponse {
        Result::from(
            state
                .pool
                .evolution_chain()
                .get_pokemon_species_by_evolution_chain_id(id, lang),
        )
    }

    ApiRouter::new()
        .nest(
            "/evolution-chains",
            api_routers::<EvolutionChain, _, _>(|state| state.pool.evolution_chain()).api_route(
                "/{id}/pokemon-species",
                get_with(get_evolution_chain_pokemon_species, |op| {
                    op.tag("evolution")
                        .tag("pokemon")
                        .description("Get a list of pokemon species by evolution chain")
                        .response_with::<200, Json<PaginatedResource<Languaged<PokemonSpecies>>>, _>(|o| {
                            o
                        })
                }),
            ),
        )
        .nest(
            "/evolution-triggers",
            api_routers::<EvolutionTrigger, _, _>(|state| state.pool.evolution_trigger()),
        )
}

fn location_routers() -> ApiRouter<AppState> {
    use pokemonle_lib::model::{Location, LocationArea, Region};
    ApiRouter::new()
        .nest(
            "/locations",
            api_languaged_routers::<Location, _, _>(|state| state.pool.location()),
        )
        .nest(
            "/location-areas",
            api_routers::<LocationArea, _, _>(|state| state.pool.location_area()),
        )
        .nest(
            "/regions",
            api_routers::<Region, _, _>(|state| state.pool.region()),
        )
}

fn move_routers() -> ApiRouter<AppState> {
    use pokemonle_lib::model::Move;

    async fn get_move_pokemons(
        State(state): State<AppState>,
        Path(Resource { id }): Path<Resource>,
        Query(Language { lang }): Query<Language>,
        Query(VersionGroup { version_group }): Query<VersionGroup>,
    ) -> impl IntoApiResponse {
        Result::from(
            state
                .pool
                .r#move()
                .get_move_pokemons(id, lang, version_group),
        )
    }

    ApiRouter::new().nest(
        "/moves",
        api_languaged_routers::<Move, _, _>(|state| state.pool.r#move())
            .api_route(
                "/{id}/pokemons",
                get_with(get_move_pokemons, |op| {
                    op.tag("move")
                        .tag("pokemon")
                        .description("Get a list of pokemons by move")
                        .response_with::<200, Json<PaginatedResource<Languaged<Pokemon>>>, _>(|o| o)
                }),
            )
            .nest(
                "/{id}/flavor-text",
                api_flavor_text_routers_with_transform::<Move, _, _, _>(
                    |state| state.pool.r#move(),
                    |op| op.tag("move"),
                ),
            ),
    )
}

pub fn routers() -> ApiRouter<AppState> {
    use pokemonle_lib::model::{Pokedex, Version, VersionGroup};

    ApiRouter::new()
        .merge(ability::routers())
        .merge(berry_routers())
        .merge(contest_routers())
        .merge(encounter_routers())
        .merge(evolution_routers())
        .nest(
            "/generations",
            api_languaged_routers::<Generation, _, _>(|state| state.pool.generation()),
        )
        .merge(item_routers())
        .merge(language::routers())
        .merge(location_routers())
        .merge(move_routers())
        .nest(
            "/pokedexes",
            api_routers::<Pokedex, _, _>(|state| state.pool.pokedex()),
        )
        .merge(pokemon::routers())
        .nest(
            "/types",
            api_languaged_routers::<Type, _, _>(|state| state.pool.r#type()),
        )
        .nest(
            "/versions",
            api_languaged_routers::<Version, _, _>(|state| state.pool.version()),
        )
        .nest(
            "/version-groups",
            api_languaged_routers::<VersionGroup, _, _>(|state| state.pool.version_group()),
        )
}
