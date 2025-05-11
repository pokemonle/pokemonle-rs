use super::AppState;

use aide::axum::{routing::get_with, ApiRouter, IntoApiResponse};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use pokemonle_lib::{database::pagination::PaginatedResource, model::LanguageName};

pub fn routers() -> ApiRouter<AppState> {
    ApiRouter::new().api_route(
        "/local-languages",
        get_with(get_local_lanuages, |op| {
            op.tag("language")
                .response_with::<200, Json<PaginatedResource<LanguageName>>, _>(|o| {
                    o.description("Get all local languages")
                })
        }),
    )
}

async fn get_local_lanuages(State(state): State<AppState>) -> impl IntoApiResponse {
    let languages = state.pool.language().get_local_lanuages();
    (StatusCode::OK, Json(languages)).into_response()
}
