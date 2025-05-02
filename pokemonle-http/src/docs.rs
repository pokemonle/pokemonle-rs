use std::sync::Arc;

use aide::{
    axum::{
        routing::{get, get_with},
        ApiRouter, IntoApiResponse,
    },
    openapi::OpenApi,
    scalar::Scalar,
};
use axum::{response::IntoResponse, Extension, Json};

use crate::v1::AppState;

pub fn routes() -> ApiRouter<AppState> {
    aide::generate::infer_responses(true);

    let router = ApiRouter::new()
        .route(
            "/",
            get_with(
                Scalar::new("/docs/private/api.json")
                    .with_title("Aide Axum")
                    .axum_handler(),
                |op| op.description("This documentation page."),
            ),
            // |p| p.security_requirement("ApiKey"),
        )
        .route("/private/api.json", get(serve_docs));

    aide::generate::infer_responses(false);

    router
}

async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(api).into_response()
}
