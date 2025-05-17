mod ability;
mod item;
// mod language;
// mod r#move;
// mod openapi;
mod pokemon;
mod router;

use aide::axum::ApiRouter;
use router::ResourceRouter;

use pokemonle_lib::database::DatabaseClient;

#[derive(Clone)]
pub struct AppState {
    pub pool: DatabaseClient,
}

pub fn routers() -> ApiRouter<AppState> {
    use pokemonle_lib::database::entity::prelude::*;

    ApiRouter::new()
        .nest("/abilities", ability::routers())
        .nest(
            "/languages",
            Languages::routers_with(|op| op.tag("language")),
        )
        .nest("/berries", Berries::routers_with(|op| op.tag("berry")))
        .nest(
            "/encounters",
            Encounters::routers_with(|op| op.tag("encounter")),
        )
        .nest(
            "/evolution-chains",
            EvolutionChains::routers_with(|op| op.tag("evolution")),
        )
        .nest(
            "/evolution-triggers",
            EvolutionTriggers::routers_with(|op| op.tag("evolution")),
        )
        .nest("/items", item::routers())
        .nest(
            "/item-categories",
            ItemCategories::routers_with(|op| op.tag("item")),
        )
        .nest(
            "/item-pockets",
            ItemPockets::routers_with(|op| op.tag("item")),
        )
        .nest("/pokemon-species", pokemon::routers())
}
