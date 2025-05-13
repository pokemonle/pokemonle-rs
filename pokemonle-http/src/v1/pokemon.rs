use aide::axum::routing::get_with;
use aide::axum::{ApiRouter, IntoApiResponse};
use aide::transform::TransformOperation;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use pokemonle_lib::database::pagination::PaginatedResource;
use pokemonle_lib::model::{Ability, PokemonColor};

use crate::v1::router::{api_flavor_text_routers_with_transform, api_languaged_routers};

use super::router::Language;
use super::{AppState, Resource};

pub fn routers() -> ApiRouter<AppState> {
    use pokemonle_lib::model::{Pokemon, PokemonSpecies};

    ApiRouter::new()
        .api_route(
            "/pokemon/identifiers",
            get_with(get_pokemon_identifiers, get_pokemon_identifiers_docs),
        )
        .nest(
            "/pokemon",
            api_languaged_routers::<Pokemon, _, _>(|state| state.pool.pokemon()),
        )
        .nest(
            "/pokemon-colors",
            api_languaged_routers::<PokemonColor, _, _>(|state| state.pool.pokemon_color()),
        )
        .nest(
            "/pokemon-species",
            api_languaged_routers::<PokemonSpecies, _, _>(|state| state.pool.pokemon_specie())
                .nest(
                    "/{id}/flavor-text",
                    api_flavor_text_routers_with_transform::<PokemonSpecies, _, _, _>(
                        |state| state.pool.pokemon_specie(),
                        |op| op.tag("pokemon-species"),
                    ),
                ),
        )
        .api_route(
            "/pokemon/{id}/abilities",
            get_with(get_pokemon_abilities, |op| {
                op.tag("pokemon")
                    .response_with::<200, Json<PaginatedResource<Ability>>, _>(|o| o)
            }),
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

/// return pokemon abilities
async fn get_pokemon_abilities(
    State(state): State<AppState>,
    Path(Resource { id }): Path<Resource>,
    Query(Language { lang }): Query<Language>,
) -> impl IntoApiResponse {
    let abilities = state.pool.pokemon().get_pokemon_abilities(id, lang);
    (StatusCode::OK, Json(abilities))
}
