// mod ability;
// mod language;
// mod r#move;
// mod openapi;
// mod pokemon;
mod resource;
mod router;

use aide::axum::ApiRouter;
use router::ResourceRouter;

use pokemonle_lib::database::DatabaseClient;

#[derive(Clone)]
pub struct AppState {
    pub pool: DatabaseClient,
}

pub fn routers() -> ApiRouter<AppState> {
    use pokemonle_lib::database::entry::{
        abilities::Entity as Ability, berries::Entity as Berry, encounters::Entity as Encounter,
        evolution_chains::Entity as EvolutionChain, evolution_triggers::Entity as EvolutionTrigger,
        item_categories::Entity as ItemCategory, item_pockets::Entity as ItemPocket,
        items::Entity as Item, languages::Entity as Language,
    };

    ApiRouter::new()
        .nest("/abilities", Ability::routers_with(|op| op.tag("ability")))
        .nest(
            "/languages",
            Language::routers_with(|op| op.tag("language")),
        )
        .nest("/berries", Berry::routers_with(|op| op.tag("berry")))
        .nest(
            "/encounters",
            Encounter::routers_with(|op| op.tag("encounter")),
        )
        .nest(
            "/evolution-chains",
            EvolutionChain::routers_with(|op| op.tag("evolution")),
        )
        .nest(
            "/evolution-triggers",
            EvolutionTrigger::routers_with(|op| op.tag("evolution")),
        )
        .nest("/items", Item::routers_with(|op| op.tag("item")))
        .nest(
            "/item-categories",
            ItemCategory::routers_with(|op| op.tag("item")),
        )
        .nest(
            "/item-pockets",
            ItemPocket::routers_with(|op| op.tag("item")),
        )
}
