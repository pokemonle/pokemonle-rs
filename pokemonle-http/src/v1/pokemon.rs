use aide::axum::routing::get_with;
use aide::axum::ApiRouter;
use aide::transform::TransformOperation;
use axum::extract::State;
use axum::Json;

use crate::v1::resource::api_routers;

use super::AppState;

pub fn routers() -> ApiRouter<AppState> {
    use pokemonle_lib::model::{Pokemon, PokemonSpecies};

    ApiRouter::new()
        .api_route(
            "/pokemon/identifiers",
            get_with(get_pokemon_identifiers, get_pokemon_identifiers_docs),
        )
        .nest(
            "/pokemon",
            api_routers::<Pokemon, _, _>(|state| state.pool.pokemon()),
        )
        .nest(
            "/pokemon-species",
            api_routers::<PokemonSpecies, _, _>(|state| state.pool.pokemon_specie()),
        )
}

/// 获取所有宝可梦的标识符列表
///
/// 返回一个包含所有宝可梦标识符的数组
async fn get_pokemon_identifiers(State(state): State<AppState>) -> Json<Vec<String>> {
    let identifiers = state.pool.pokemon().get_all_identifiers();
    Json(identifiers)
}

fn get_pokemon_identifiers_docs(op: TransformOperation) -> TransformOperation {
    op.tag("pokemon")
}
