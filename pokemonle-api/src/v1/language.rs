use super::{router::api_languaged_routers, AppState};
use crate::error::Result;

use aide::axum::{routing::get_with, ApiRouter, IntoApiResponse};
use axum::{extract::State, Json};
use pokemonle_lib::{
    database::pagination::PaginatedResource,
    model::{Language, LanguageName},
};

pub fn routers() -> ApiRouter<AppState> {
    ApiRouter::new()
        .nest(
            "/languages",
            api_languaged_routers::<Language, _, _>(|state| state.pool.language()),
        )
        .api_route(
            "/local-languages",
            get_with(get_local_lanuages, |op| {
                op.tag("language")
                    .response_with::<200, Json<PaginatedResource<LanguageName>>, _>(|o| o)
            }),
        )
}

async fn get_local_lanuages(State(state): State<AppState>) -> impl IntoApiResponse {
    Result::from(state.pool.language().get_local_lanuages())
}
