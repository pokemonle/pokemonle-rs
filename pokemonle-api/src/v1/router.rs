use crate::error::Result;
use aide::{
    axum::{routing::get_with, ApiRouter, IntoApiResponse},
    transform::TransformOperation,
};

use axum::{
    extract::{Path, Query, State},
    Json,
};

use pokemonle_lib::{
    database::r#trait::ResourceHandler,
    sea_orm::{EntityTrait, FromQueryResult, PrimaryKeyTrait},
    types::request::PaginateQuery,
};

use pokemonle_lib::types::{request::ResourceId, response::PaginatedResource};
use schemars::JsonSchema;
use serde::Serialize;

use super::AppState;

pub trait ResourceRouter<T> {
    #[allow(dead_code)]
    fn routers() -> ApiRouter<AppState> {
        Self::routers_with(|op| op)
    }

    fn routers_with(
        f: impl FnOnce(TransformOperation) -> TransformOperation + Copy,
    ) -> ApiRouter<AppState>;
}

pub trait LocalizedResourceRouter<T, N> {
    #[allow(dead_code)]
    fn routers() -> ApiRouter<AppState> {
        Self::routers_with(|op| op)
    }

    fn routers_with(
        f: impl FnOnce(TransformOperation) -> TransformOperation + Copy,
    ) -> ApiRouter<AppState>;
}

impl<T> ResourceRouter<T> for T
where
    T: EntityTrait + 'static,
    T::PrimaryKey: PrimaryKeyTrait<ValueType = i32>,
    T::Model: Serialize + JsonSchema + FromQueryResult + Sized + Send + Sync,
{
    fn routers_with(
        f: impl FnOnce(TransformOperation) -> TransformOperation + Copy,
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
            Result::from(handler.list_with_pagination(page, per_page).await)
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
