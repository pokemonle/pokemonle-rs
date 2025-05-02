mod resource;
mod response;

use aide::axum::ApiRouter;

use pokemonle_lib::{
    database::handler::DatabaseClientPooled,
    model::{Generation, Item, ItemCategory, ItemPocket, Type},
};
use resource::{api_routers, api_routers_with_transform};

pub use response::ListResponse;

#[derive(Clone)]
pub struct AppState {
    pub pool: DatabaseClientPooled,
}

pub fn routers() -> ApiRouter<AppState> {
    use pokemonle_lib::model::{Ability, Language, Version, VersionGroup};

    ApiRouter::new()
        .nest(
            "/languages",
            api_routers::<Language, _, _>(|state| state.pool.language_handler()),
        )
        .nest(
            "/versions",
            api_routers::<Version, _, _>(|state| state.pool.version_handler()),
        )
        .nest(
            "/version-groups",
            api_routers::<VersionGroup, _, _>(|state| state.pool.version_group_handler()),
        )
        .nest(
            "/abilities",
            api_routers::<Ability, _, _>(|state| state.pool.ability_handler()),
        )
        .nest(
            "/generations",
            api_routers::<Generation, _, _>(|state| state.pool.generation_handler()),
        )
        .nest(
            "/items",
            api_routers::<Item, _, _>(|state| state.pool.item_handler()),
        )
        .nest(
            "/item-categories",
            api_routers_with_transform::<ItemCategory, _, _, _>(
                |state| state.pool.item_handler().category_handler(),
                |o| o.tag("item"),
            ),
        )
        .nest(
            "/item-pockets",
            api_routers_with_transform::<ItemPocket, _, _, _>(
                |state| state.pool.item_handler().pocket_handler(),
                |o| o.tag("item"),
            ),
        )
        .nest(
            "/types",
            api_routers::<Type, _, _>(|state| state.pool.type_handler()),
        )
}
