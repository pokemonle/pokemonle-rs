use aide::axum::ApiRouter;
use pokemonle_lib::model::{PokemonColor, PokemonSpecieDetail, PokemonSpecies};

use crate::v1::resource::api_routers;

use super::AppState;

pub fn routers() -> ApiRouter<AppState> {
    use pokemonle_lib::model::Pokemon;

    ApiRouter::new()
        .nest(
            "/pokemons",
            api_routers::<Pokemon, _, _>(|state| state.pool.pokemon_handler()),
        )
        .nest(
            "/pokemon-species",
            api_routers::<PokemonSpecieDetail, _, _>(|state| {
                state.pool.pokemon_handler().specie_handler()
            }),
        )
}
