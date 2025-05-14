use crate::error::Result;
use aide::axum::{routing::get_with, ApiRouter, IntoApiResponse};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use pokemonle_lib::{
    database::pagination::PaginatedResource,
    model::{Ability, Pokemon},
};

use super::{
    router::{api_flavor_text_routers_with_transform, api_languaged_routers, Language},
    AppState, Resource,
};

async fn get_ablitity_pokemons(
    State(state): State<AppState>,
    Path(Resource { id }): Path<Resource>,
    Query(Language { lang }): Query<Language>,
) -> impl IntoApiResponse {
    Result::from(state.pool.pokemon().list_by_ability(id, lang))
}

pub fn routers() -> ApiRouter<AppState> {
    ApiRouter::new().nest(
        "/abilities",
        api_languaged_routers::<Ability, _, _>(|state| state.pool.ability())
            .nest(
                "/{id}/flavor-text",
                api_flavor_text_routers_with_transform::<Ability, _, _, _>(
                    |state| state.pool.ability(),
                    |op| op.tag("ability"),
                ),
            )
            .api_route(
                "/{id}/pokemons",
                get_with(get_ablitity_pokemons, |op| {
                    op.tag("ability")
                        .tag("pokemon")
                        .description("Get a list of pokemons by ability")
                        .response_with::<200, Json<PaginatedResource<Pokemon>>, _>(|o| o)
                }),
            ),
    )
}
