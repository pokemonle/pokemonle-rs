mod resource;
mod response;

use aide::axum::ApiRouter;

use pokemonle_lib::database::handler::DatabaseClientPooled;
use resource::api_routers;

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
            api_routers::<Language, _, _>(|state: AppState| state.pool.language_handler()),
        )
        .nest(
            "/versions",
            api_routers::<Version, _, _>(|state: AppState| state.pool.version_handler()),
        )
        .nest(
            "/version-groups",
            api_routers::<VersionGroup, _, _>(|state: AppState| state.pool.version_group_handler()),
        )
        .nest(
            "/abilities",
            api_routers::<Ability, _, _>(|state: AppState| state.pool.ability_handler()),
        )
}
