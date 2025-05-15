use crate::error::Result;
use aide::{
    axum::{routing::get_with, ApiRouter, IntoApiResponse},
    transform::TransformOperation,
    OperationOutput,
};

use axum::{
    extract::{Path, Query, State},
    Json,
};
use pokemonle_lib::{
    database::{
        handler::{DatabaseHandler, DatabaseHandlerWithFlavorText, DatabaseHandlerWithLocale},
        pagination::{Paginated, PaginatedResource},
    },
    model::{Languaged, ResourceDescription},
};
use pokemonle_trait::StructName;
use schemars::JsonSchema;
use serde::Serialize;

use pokemonle_lib::types::param::{Language, Resource, SearchQuery};

use super::AppState;

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

        Result::from(handler.get_all_resources(pagination))
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

        crate::error::Result::from(handler.get_resource_by_id(resource.id))
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

pub fn api_languaged_routers<T, H, F>(handler_fn: F) -> ApiRouter<AppState>
where
    T: StructName + OperationOutput + Serialize + JsonSchema + Clone + Send + Sync + 'static,
    <T as OperationOutput>::Inner: Serialize + From<T>,
    H: DatabaseHandlerWithLocale<Resource = T> + Sync + 'static,
    F: Fn(AppState) -> H + Clone + Copy + Send + Sync + 'static,
{
    api_languaged_routers_with_transform(handler_fn, |o| o)
}

pub fn api_languaged_routers_with_transform<T, H, F, O>(
    handler_fn: F,
    transform: O,
) -> ApiRouter<AppState>
where
    T: StructName + OperationOutput + Serialize + JsonSchema + Clone + Send + Sync + 'static,
    <T as OperationOutput>::Inner: Serialize + From<T>,
    H: DatabaseHandlerWithLocale<Resource = T> + Sync + 'static,
    F: Fn(AppState) -> H + Clone + Copy + Send + Sync + 'static,
    O: FnOnce(TransformOperation) -> TransformOperation + Clone + Copy,
{
    use super::openapi::{get_item_by_id_docs, list_items_docs};

    async fn list<T, H>(
        State(state): State<AppState>,
        Query(Language { lang }): Query<Language>,
        Query(pagination): Query<Paginated>,
        Query(search): Query<SearchQuery>,
        handle_fn: impl Fn(AppState) -> H,
    ) -> impl IntoApiResponse
    where
        T: Serialize + Clone + Send + Sync + StructName,
        H: DatabaseHandlerWithLocale<Resource = T>,
    {
        let handler = handle_fn(state);

        Result::from(handler.get_all_resources_with_locale(pagination, lang, search.q))
    }

    async fn get<T, H>(
        State(state): State<AppState>,
        Query(Language { lang }): Query<Language>,
        Path(resource): Path<Resource>,
        handler_fn: impl Fn(AppState) -> H,
    ) -> impl IntoApiResponse
    where
        T: Serialize + Clone + Send + Sync + StructName,
        H: DatabaseHandlerWithLocale<Resource = T>,
    {
        let handler = handler_fn(state);

        Result::from(handler.get_resource_by_id_with_locale(resource.id, lang))
    }

    ApiRouter::new()
        .api_route(
            "/",
            get_with(
                move |(state, lang, pagination, search)| {
                    list::<T, H>(state, lang, pagination, search, handler_fn)
                },
                move |op| transform(list_items_docs::<Languaged<T>>(op)),
            ),
        )
        .api_route(
            "/{id}",
            get_with(
                move |(state, lang, id)| get::<T, H>(state, lang, id, handler_fn),
                // get_item_by_id_docs with transform
                move |op| transform(get_item_by_id_docs::<Languaged<T>>(op)),
            ),
        )
}

pub fn api_flavor_text_routers_with_transform<T, H, F, O>(
    handler_fn: F,
    transform: O,
) -> ApiRouter<AppState>
where
    T: StructName + OperationOutput + Serialize + JsonSchema + Clone + Send + Sync + 'static,
    <T as OperationOutput>::Inner: Serialize + From<T>,
    H: DatabaseHandlerWithFlavorText + Sync + 'static,
    F: Fn(AppState) -> H + Clone + Copy + Send + Sync + 'static,
    O: FnOnce(TransformOperation) -> TransformOperation + Clone + Copy,
{
    async fn list<H>(
        State(state): State<AppState>,
        Path(resource): Path<Resource>,
        Query(Language { lang }): Query<Language>,
        Query(pagination): Query<Paginated>,
        handle_fn: impl Fn(AppState) -> H,
    ) -> impl IntoApiResponse
    where
        H: DatabaseHandlerWithFlavorText,
    {
        let handler = handle_fn(state);

        Result::from(handler.get_all_resources_with_flavor_text(resource.id, pagination, lang))
    }

    async fn get<H>(
        State(state): State<AppState>,
        Query(Language { lang }): Query<Language>,
        Path(resource): Path<Resource>,
        // Path(version): Path<Version>,
        handler_fn: impl Fn(AppState) -> H,
    ) -> impl IntoApiResponse
    where
        H: DatabaseHandlerWithFlavorText,
    {
        let handler = handler_fn(state);

        Result::from(handler.get_latest_flavor_text(resource.id, lang))
    }

    ApiRouter::new()
        .api_route(
            "/",
            get_with(
                move |(state, resource, lang, pagination)| {
                    list::<H>(state, resource, lang, pagination, handler_fn)
                },
                move |op| {
                    transform(
                        op.response_with::<200, Json<PaginatedResource<ResourceDescription>>, _>(
                            |r| r.description("return paginated flavor text"),
                        ),
                    )
                },
            ),
        )
        .api_route(
            "/latest",
            get_with(
                move |(state, lang, id)| get::<H>(state, lang, id, handler_fn),
                // get_item_by_id_docs with transform
                move |op| {
                    transform(op.response_with::<200, Json<ResourceDescription>, _>(|r| {
                        r.description("return latest flavor text")
                    }))
                },
            ),
        )
}
