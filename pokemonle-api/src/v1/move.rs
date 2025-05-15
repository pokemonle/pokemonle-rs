use crate::error::Result;
use aide::axum::{routing::get_with, ApiRouter, IntoApiResponse};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use pokemonle_lib::{
    database::pagination::PaginatedResource,
    model::{Languaged, Pokemon},
    types::param::{Language, Resource, VersionGroup},
};

use super::{
    router::{api_flavor_text_routers_with_transform, api_languaged_routers},
    AppState,
};

pub fn routers() -> ApiRouter<AppState> {
    use pokemonle_lib::model::Move;

    async fn get_move_pokemons(
        State(state): State<AppState>,
        Path(Resource { id }): Path<Resource>,
        Query(Language { lang }): Query<Language>,
        Query(VersionGroup { version_group }): Query<VersionGroup>,
    ) -> impl IntoApiResponse {
        Result::from(
            state
                .pool
                .r#move()
                .get_move_pokemons(id, lang, version_group),
        )
    }

    ApiRouter::new().nest(
        "/moves",
        api_languaged_routers::<Move, _, _>(|state| state.pool.r#move())
            .api_route(
                "/{id}/pokemons",
                get_with(get_move_pokemons, |op| {
                    op.tag("move")
                        .tag("pokemon")
                        .description("Get a list of pokemons by move")
                        .response_with::<200, Json<PaginatedResource<Languaged<Pokemon>>>, _>(|o| o)
                }),
            )
            .nest(
                "/{id}/flavor-text",
                api_flavor_text_routers_with_transform::<Move, _, _, _>(
                    |state| state.pool.r#move(),
                    |op| op.tag("move"),
                ),
            ),
    )
}
