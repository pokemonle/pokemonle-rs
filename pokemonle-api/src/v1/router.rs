use crate::error::Error;
use crate::error::Result;
use aide::{
    axum::{routing::get_with, ApiRouter, IntoApiResponse},
    transform::TransformOperation,
    OperationOutput,
};

use axum::http::StatusCode;
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};

use pokemonle_lib::database::r#trait::LocalizedEntity;
use pokemonle_lib::types::WithName;
use pokemonle_lib::{
    database::r#trait::{LocalizedResourceHandler, ResourceHandler},
    sea_orm::{EntityTrait, FromQueryResult, PrimaryKeyTrait},
    types::request::PaginateQuery,
};

use pokemonle_lib::types::{
    request::{Language, ResourceId, SearchQuery},
    response::PaginatedResource,
};
use schemars::JsonSchema;
use serde::Serialize;

use super::AppState;

pub trait ResourceRouter<T> {
    fn routers() -> ApiRouter<AppState> {
        Self::routers_with(|op| op)
    }

    fn routers_with(
        f: impl FnOnce(TransformOperation) -> TransformOperation + Clone + Copy,
    ) -> ApiRouter<AppState>;
}

impl<T> ResourceRouter<T> for T
where
    T: EntityTrait + 'static,
    T::PrimaryKey: PrimaryKeyTrait<ValueType = i32>,
    T::Model: Serialize + JsonSchema + FromQueryResult + Sized + Send + Sync,
{
    fn routers_with(
        f: impl FnOnce(TransformOperation) -> TransformOperation + Clone + Copy,
    ) -> ApiRouter<AppState> {
        async fn list_resource<T>(
            State(state): State<AppState>,
            Query(PaginateQuery { page, per_page }): Query<PaginateQuery>,
        ) -> impl IntoApiResponse
        where
            T: EntityTrait + 'static,
            T::PrimaryKey: PrimaryKeyTrait<ValueType = i32>,
            T::Model: Serialize + JsonSchema + FromQueryResult + Sized + Send + Sync,
        {
            let handler: &dyn ResourceHandler<T> = &state.pool;
            Result::from(
                handler
                    .list_with_pagination(page as u64, per_page as u64)
                    .await,
            )
        }

        async fn get_resource<T>(
            State(state): State<AppState>,
            Path(ResourceId { id }): Path<ResourceId>,
        ) -> impl IntoApiResponse
        where
            T: EntityTrait + 'static,
            T::PrimaryKey: PrimaryKeyTrait<ValueType = i32>,
            T::Model: Serialize + Sync,
        {
            let handler: &dyn ResourceHandler<T> = &state.pool;
            Result::from(handler.get_by_id(id).await)
        }

        ApiRouter::new()
            .api_route(
                "/",
                get_with(list_resource::<T>, |op| {
                    f(op.response_with::<200, Json<PaginatedResource<T::Model>>, _>(|o| o))
                }),
            )
            .api_route(
                "/{id}",
                get_with(get_resource::<T>, |op| {
                    f(op.response_with::<200, Json<T::Model>, _>(|o| o))
                }),
            )
    }
}

impl<T> ResourceRouter<T> for WithName<T>
where
    T: LocalizedEntity + 'static,
    T::PrimaryKey: PrimaryKeyTrait<ValueType = i32>,
    T::Model: Serialize + JsonSchema + FromQueryResult + Sized + Send + Sync,
{
    fn routers_with(
        f: impl FnOnce(TransformOperation) -> TransformOperation + Clone + Copy,
    ) -> ApiRouter<AppState> {
        async fn list_resource<T>(
            State(state): State<AppState>,
            Query(PaginateQuery { page, per_page }): Query<PaginateQuery>,
            Query(Language { lang }): Query<Language>,
        ) -> impl IntoApiResponse
        where
            T: LocalizedEntity + 'static,
            T::PrimaryKey: PrimaryKeyTrait<ValueType = i32>,
            T::Model: Serialize + JsonSchema + FromQueryResult + Sized + Send + Sync,
        {
            let handler: &dyn LocalizedResourceHandler<T> = &state.pool;
            Result::from(
                handler
                    .list_with_pagination(page as u64, per_page as u64, lang)
                    .await,
            )
        }

        async fn get_resource<T>(
            State(state): State<AppState>,
            Path(ResourceId { id }): Path<ResourceId>,
            Query(Language { lang }): Query<Language>,
        ) -> impl IntoApiResponse
        where
            T: LocalizedEntity + 'static,
            T::PrimaryKey: PrimaryKeyTrait<ValueType = i32>,
            T::Model: Serialize + Sync,
        {
            let handler: &dyn LocalizedResourceHandler<T> = &state.pool;
            Result::from(handler.get_by_id(id, lang).await)
        }

        ApiRouter::new()
            .api_route(
                "/",
                get_with(list_resource::<T>, |op| {
                    f(op.response_with::<200, Json<PaginatedResource<T::Model>>, _>(|o| o))
                }),
            )
            .api_route(
                "/{id}",
                get_with(get_resource::<T>, |op| {
                    f(op.response_with::<200, Json<T::Model>, _>(|o| o))
                }),
            )
    }
}
