mod game;
mod language;
mod openapi;
mod pokemon;
mod resource;
mod response;
mod router;

use router::{api_flavor_text_routers_with_transform, Language};
use std::sync::Arc;

use aide::axum::{routing::get_with, ApiRouter, IntoApiResponse};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use pokemonle_lib::{
    crypto::Crypto,
    database::{handler::DatabaseClientPooled, pagination::PaginatedResource},
    model::{Generation, Pokemon, Type},
};
use router::{api_languaged_routers, api_routers};
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Clone)]
pub struct AppState {
    pub pool: DatabaseClientPooled,
    pub crypto: Arc<dyn Crypto>,
}

#[derive(Deserialize, JsonSchema)]
pub struct Resource {
    id: i32,
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
    ApiRouter::new().nest(
        "/moves",
        api_languaged_routers::<Move, _, _>(|state| state.pool.r#move()),
    )
    // .nest(
    //     "/move-targets",
    //     api_routers::<MoveTarget, _, _>(|state| state.pool.move_target()),
    // )
}

async fn get_ablitity_pokemons(
    State(state): State<AppState>,
    Path(Resource { id }): Path<Resource>,
    Query(Language { lang }): Query<Language>,
) -> impl IntoApiResponse {
    let pokemons = state.pool.pokemon().list_by_ability(id, lang);
    (StatusCode::OK, Json(pokemons))
}

pub fn routers() -> ApiRouter<AppState> {
    use pokemonle_lib::model::{Ability, Language, Pokedex, Version, VersionGroup};

    ApiRouter::new()
        .nest(
            "/abilities",
            api_languaged_routers::<Ability, _, _>(|state| state.pool.ability()).api_route(
                "/{id}/pokemons",
                get_with(get_ablitity_pokemons, |op| {
                    op.tag("ability")
                        .tag("pokemon")
                        .description("Get a list of pokemons by ability")
                        .response_with::<200, Json<PaginatedResource<Pokemon>>, _>(|o| {
                            o.description("example")
                        })
                }),
            ),
        )
        .merge(berry_routers())
        .merge(contest_routers())
        .merge(encounter_routers())
        .nest("/game", game::routers())
        .nest(
            "/generations",
            api_routers::<Generation, _, _>(|state| state.pool.generation()),
        )
        .merge(item_routers())
        .nest(
            "/languages",
            api_routers::<Language, _, _>(|state| state.pool.language()),
        )
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
            api_routers::<VersionGroup, _, _>(|state| state.pool.version_group()),
        )
}
