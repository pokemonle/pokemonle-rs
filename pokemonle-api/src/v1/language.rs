use crate::error::Result;
use aide::axum::{routing::get_with, ApiRouter, IntoApiResponse};
use axum::extract::{Path, Query, State};
use pokemonle_lib::{
    database::{
        entity::{self, language_names, prelude::*},
        r#trait::LocalizedResourceHandler,
    },
    types::prelude::*,
};

use super::{router::response_with, AppState};

async fn list_languages_with_pagination(
    State(state): State<AppState>,
    Query(PaginateQuery { page, per_page }): Query<PaginateQuery>,
    Query(Language { lang }): Query<Language>,
    Query(SearchQuery { q }): Query<SearchQuery>,
) -> impl IntoApiResponse {
    let handler: &dyn LocalizedResourceHandler<Languages, LanguageNames> = &state.pool;
    Result::from(handler.list_with_pagination(page, per_page, lang, q).await)
}

async fn get_language_by_id(
    State(state): State<AppState>,
    Path(ResourceId { id }): Path<ResourceId>,
    Query(Language { lang }): Query<Language>,
) -> impl IntoApiResponse {
    let handler: &dyn LocalizedResourceHandler<Languages, LanguageNames> = &state.pool;
    Result::from(handler.get_by_id(id, lang).await)
}

pub fn routers() -> ApiRouter<AppState> {
    use entity::languages::Model;
    ApiRouter::new()
        .api_route(
            "/languages",
            get_with(list_languages_with_pagination, |op| {
                response_with::<PaginatedResource<WithName<Model>>>(op).tag("language")
            }),
        )
        .api_route(
            "/languages/{id}",
            get_with(get_language_by_id, |op| {
                response_with::<WithName<Model>>(op).tag("language")
            }),
        )
        .api_route(
            "/local-languages",
            get_with(get_local_lanuages, |op| {
                response_with::<PaginatedResource<language_names::Model>>(op).tag("language")
            }),
        )
}

async fn get_local_lanuages(State(state): State<AppState>) -> impl IntoApiResponse {
    Result::from(state.pool.get_local_languages().await)
}
