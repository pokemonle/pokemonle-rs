mod ability;
mod contest;
mod evo;
mod item;
mod language;
mod r#move;
// mod openapi;
mod location;
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
        .nest("/berries", Berries::routers_with(|op| op.tag("berry")))
        .nest(
            "/berry-firmness",
            BerryFirmness::routers_with(|op| op.tag("berry")),
        )
        .merge(contest::routers())
        .nest(
            "/encounters",
            Encounters::routers_with(|op| op.tag("encounter")),
        )
        .merge(evo::routers())
        .nest("/items", item::routers())
        .nest(
            "/item-categories",
            ItemCategories::routers_with(|op| op.tag("item")),
        )
        .nest(
            "/item-pockets",
            ItemPockets::routers_with(|op| op.tag("item")),
        )
        .merge(language::routers())
        .merge(location::routers())
        .nest("/moves", r#move::routers())
        .merge(pokemon::routers())
}
