mod game;
mod pokemon;
mod resource;
mod response;

use std::sync::Arc;

use aide::axum::ApiRouter;

use pokemonle_lib::{
    crypto::Crypto,
    database::handler::DatabaseClientPooled,
    model::{Generation, Type},
};
use resource::api_routers;

#[derive(Clone)]
pub struct AppState {
    pub pool: DatabaseClientPooled,
    pub crypto: Arc<dyn Crypto>,
}

fn item_routers() -> ApiRouter<AppState> {
    use pokemonle_lib::model::{Item, ItemCategory, ItemPocket};
    ApiRouter::new()
        .nest(
            "/items",
            api_routers::<Item, _, _>(|state| state.pool.item()),
        )
        .nest(
            "/item-categories",
            api_routers::<ItemCategory, _, _>(|state| state.pool.item_category()),
        )
        .nest(
            "/item-pockets",
            api_routers::<ItemPocket, _, _>(|state| state.pool.item_pocket()),
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

pub fn routers() -> ApiRouter<AppState> {
    use pokemonle_lib::model::{Ability, Language, Version, VersionGroup};

    ApiRouter::new()
        .nest(
            "/abilities",
            api_routers::<Ability, _, _>(|state| state.pool.ability()),
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
        .merge(pokemon::routers())
        .nest(
            "/types",
            api_routers::<Type, _, _>(|state| state.pool.r#type()),
        )
        .nest(
            "/versions",
            api_routers::<Version, _, _>(|state| state.pool.version()),
        )
        .nest(
            "/version-groups",
            api_routers::<VersionGroup, _, _>(|state| state.pool.version_group()),
        )
}
