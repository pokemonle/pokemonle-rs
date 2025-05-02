mod ability;
mod language;
mod response;
mod version;
mod version_group;

use aide::axum::ApiRouter;

use pokemonle_lib::database::handler::DatabaseClientPooled;

pub use response::ListResponse;

#[derive(Clone)]
pub struct AppState {
    pub pool: DatabaseClientPooled,
}

pub fn routers() -> ApiRouter<AppState> {
    ApiRouter::new()
        .nest("/languages", language::routers())
        .nest("/versions", version::routers())
        .nest("/version-groups", version_group::routers())
        .nest("/abilities", ability::routers())
}
