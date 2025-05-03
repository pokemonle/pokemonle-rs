use aide::axum::ApiRouter;

use crate::v1::resource::api_routers;

use super::AppState;

pub fn routers() -> ApiRouter<AppState> {
    use pokemonle_lib::model::{Pokemon, PokemonColor, PokemonHabitat, PokemonShape};

    ApiRouter::new().nest(
        "/pokemons",
        api_routers::<Pokemon, _, _>(|state| state.pool.pokemon_handler()),
    )
}
