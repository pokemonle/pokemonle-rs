use crate::error::Error;
use aide::{
    axum::{routing::get_with, ApiRouter, IntoApiResponse},
    OperationOutput,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use pokemonle_lib::database::handler::DatabaseHandler;
use pokemonle_trait::StructName;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{AppState, ListResponse};

#[derive(Deserialize, JsonSchema)]
struct Resource {
    id: i32,
}

pub fn api_routers<T, H, F>(handler_fn: F) -> ApiRouter<AppState>
where
    T: StructName + OperationOutput + Serialize + JsonSchema + Clone + Send + Sync + 'static,
    <T as OperationOutput>::Inner: Serialize + From<T>,
    H: DatabaseHandler<Resource = T> + Sync + 'static,
    F: Fn(AppState) -> H + Clone + Copy + Send + Sync + 'static,
{
    use super::response::{get_item_by_id_docs, list_items_docs};

    async fn list<T, H>(
        State(state): State<AppState>,
        handle_fn: impl Fn(AppState) -> H,
    ) -> impl IntoApiResponse
    where
        T: Serialize + Clone + Send + Sync,
        H: DatabaseHandler<Resource = T>,
    {
        let handler = handle_fn(state);
        (
            StatusCode::OK,
            Json(ListResponse::new(handler.get_all_resources())),
        )
    }

    async fn get<T, H>(
        State(state): State<AppState>,
        Path(resource): Path<Resource>,
        handler_fn: impl Fn(AppState) -> H,
    ) -> impl IntoApiResponse
    where
        T: Serialize + Clone + Send + Sync + StructName,
        H: DatabaseHandler<Resource = T>,
    {
        let handler = handler_fn(state);
        let struct_name = T::struct_name();

        match handler.get_resource_by_id(resource.id) {
            Some(resource) => (StatusCode::OK, Json(resource)).into_response(),
            None => {
                let err = Error::ResourceNotFound(format!(
                    "{} with id {} not found",
                    struct_name, resource.id
                ));
                err.into_response()
            }
        }
    }

    ApiRouter::new()
        .api_route(
            "/",
            get_with(
                move |state| list::<T, H>(state, handler_fn.clone()),
                list_items_docs::<T>,
            ),
        )
        .api_route(
            "/{id}",
            get_with(
                move |state, id| get::<T, H>(state, id, handler_fn.clone()),
                get_item_by_id_docs::<T>,
            ),
        )
}
