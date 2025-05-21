use crate::error::Result;
use aide::axum::{routing::get_with, ApiRouter, IntoApiResponse};
use axum::extract::{Path, Query, State};
use pokemonle_lib::{
    database::{
        entity::{self, prelude::*},
        r#trait::LocalizedResourceHandler,
    },
    types::prelude::*,
};

use super::{router::response_with, AppState};

async fn list_moves_with_pagination(
    State(state): State<AppState>,
    Query(PaginateQuery { page, per_page }): Query<PaginateQuery>,
    Query(Language { lang }): Query<Language>,
    Query(SearchQuery { q }): Query<SearchQuery>,
) -> impl IntoApiResponse {
    let handler: &dyn LocalizedResourceHandler<Moves, MoveNames> = &state.pool;
    Result::from(handler.list_with_pagination(page, per_page, lang, q).await)
}

async fn get_move_by_id(
    State(state): State<AppState>,
    Path(ResourceId { id }): Path<ResourceId>,
    Query(Language { lang }): Query<Language>,
) -> impl IntoApiResponse {
    let handler: &dyn LocalizedResourceHandler<Moves, MoveNames> = &state.pool;
    Result::from(handler.get_by_id(id, lang).await)
}

async fn get_pokemons_by_move_id(
    State(state): State<AppState>,
    Path(ResourceId { id }): Path<ResourceId>,
    Query(PaginateQuery { page, per_page }): Query<PaginateQuery>,
    Query(Language { lang }): Query<Language>,
    Query(VersionGroup { version_group }): Query<VersionGroup>,
) -> impl IntoApiResponse {
    let handler = &state.pool;
    Result::from(
        handler
            .get_pokemons_by_move_id(id, version_group, page, per_page, lang)
            .await,
    )
}

async fn get_move_flavor_text(
    State(state): State<AppState>,
    Path(ResourceId { id }): Path<ResourceId>,
    Query(Language { lang }): Query<Language>,
    Query(VersionGroup { version_group }): Query<VersionGroup>,
) -> impl IntoApiResponse {
    let handler = &state.pool;

    Result::from(handler.get_move_flavor_text(id, version_group, lang).await)
}

pub fn routers() -> ApiRouter<AppState> {
    use entity::moves::Model;
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list_moves_with_pagination, |op| {
                response_with::<PaginatedResource<WithName<Model>>>(op).tag("move")
            }),
        )
        .api_route(
            "/{id}",
            get_with(get_move_by_id, |op| {
                response_with::<WithName<Model>>(op).tag("move")
            }),
        )
        .api_route(
            "/{id}/pokemons",
            get_with(get_pokemons_by_move_id, |op| op.tag("move")),
        )
        .api_route(
            "/{id}/flavor-text",
            get_with(get_move_flavor_text, |op| op.tag("move")),
        )
        .api_route(
            "/{id}/flavor-text/last",
            get_with(
                |state: State<AppState>,
                 resource: Path<ResourceId>,
                 lang: Query<Language>| async move {
                    let handler = &state.pool;
                    Result::from(
                        handler
                            .get_move_flavor_text(resource.id, None, lang.lang)
                            .await,
                    )
                },
                |op| response_with::<WithName<Model>>(op).tag("move"),
            ),
        )
}
