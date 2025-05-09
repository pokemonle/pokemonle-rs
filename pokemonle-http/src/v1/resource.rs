use crate::error::Error;
use aide::{
    axum::{routing::get_with, ApiRouter, IntoApiResponse},
    transform::TransformOperation,
    OperationOutput,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use pokemonle_lib::database::{handler::DatabaseHandler, pagination::Paginated};
use pokemonle_trait::StructName;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::AppState;

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
    api_routers_with_transform(handler_fn, |o| o)
}

pub fn api_routers_with_transform<T, H, F, O>(handler_fn: F, transform: O) -> ApiRouter<AppState>
where
    T: StructName + OperationOutput + Serialize + JsonSchema + Clone + Send + Sync + 'static,
    <T as OperationOutput>::Inner: Serialize + From<T>,
    H: DatabaseHandler<Resource = T> + Sync + 'static,
    F: Fn(AppState) -> H + Clone + Copy + Send + Sync + 'static,
    O: FnOnce(TransformOperation) -> TransformOperation + Clone + Copy,
{
    use super::openapi::{get_item_by_id_docs, list_items_docs};

    async fn list<T, H>(
        State(state): State<AppState>,
        Query(pagination): Query<Paginated>,
        handle_fn: impl Fn(AppState) -> H,
    ) -> impl IntoApiResponse
    where
        T: Serialize + Clone + Send + Sync,
        H: DatabaseHandler<Resource = T>,
    {
        let handler = handle_fn(state);
        (StatusCode::OK, Json(handler.get_all_resources(pagination)))
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
                move |(state, pagination)| list::<T, H>(state, pagination, handler_fn),
                move |op| transform(list_items_docs::<T>(op)),
            ),
        )
        .api_route(
            "/{id}",
            get_with(
                move |state, id| get::<T, H>(state, id, handler_fn),
                // get_item_by_id_docs with transform
                move |op| transform(get_item_by_id_docs::<T>(op)),
            ),
        )
}
