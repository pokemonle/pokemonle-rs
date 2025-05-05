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
            api_routers::<Item, _, _>(|state| state.pool.item_handler()),
        )
        .nest(
            "/item-categories",
            api_routers::<ItemCategory, _, _>(|state| state.pool.item_handler().category_handler()),
        )
        .nest(
            "/item-pockets",
            api_routers::<ItemPocket, _, _>(|state| state.pool.item_handler().pocket_handler()),
        )
}

fn berry_routers() -> ApiRouter<AppState> {
    use pokemonle_lib::model::{Berry, BerryFirmness};
    ApiRouter::new()
        .nest(
            "/berries",
            api_routers::<Berry, _, _>(|state| state.pool.berry_handler()),
        )
        .nest(
            "/berry-firmness",
            api_routers::<BerryFirmness, _, _>(|state| state.pool.berry_handler().firmness()),
        )
}

fn contest_routers() -> ApiRouter<AppState> {
    use pokemonle_lib::model::{ContestEffect, ContestType};
    ApiRouter::new()
        .nest(
            "/contest-effects",
            api_routers::<ContestEffect, _, _>(|state| state.pool.contest_effect_handler()),
        )
        .nest(
            "/contest-types",
            api_routers::<ContestType, _, _>(|state| state.pool.contest_type_handler()),
        )
}

pub fn routers() -> ApiRouter<AppState> {
    use pokemonle_lib::model::{Ability, Language, Version, VersionGroup};

    ApiRouter::new()
        .nest(
            "/abilities",
            api_routers::<Ability, _, _>(|state| state.pool.ability_handler()),
        )
        .merge(berry_routers())
        .merge(contest_routers())
        .nest("/game", game::routers())
        .nest(
            "/generations",
            api_routers::<Generation, _, _>(|state| state.pool.generation_handler()),
        )
        .merge(item_routers())
        .nest(
            "/languages",
            api_routers::<Language, _, _>(|state| state.pool.language_handler()),
        )
        .merge(pokemon::routers())
        .nest(
            "/types",
            api_routers::<Type, _, _>(|state| state.pool.type_handler()),
        )
        .nest(
            "/versions",
            api_routers::<Version, _, _>(|state| state.pool.version_handler()),
        )
        .nest(
            "/version-groups",
            api_routers::<VersionGroup, _, _>(|state| state.pool.version_group_handler()),
        )
}
