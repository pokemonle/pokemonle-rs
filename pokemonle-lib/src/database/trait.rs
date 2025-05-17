use crate::{prelude::*, types::WithName};
use async_trait::async_trait;
use sea_orm::{DbConn, EntityTrait};

use crate::types::response::PaginatedResource;

#[async_trait]
pub trait ResourceHandler<T>
where
    T: EntityTrait + Send + Sync + 'static,
    Self: Send + Sync,
{
    async fn list_with_pagination(
        &self,
        page: u64,
        limit: u64,
    ) -> Result<PaginatedResource<T::Model>>;

    async fn get_by_id(&self, id: i32) -> Result<T::Model>;
}

#[async_trait]
pub trait DBConnection: Send + Sync {
    async fn get_conn(self) -> DbConn;
}

#[async_trait]
pub trait LocalizedResourceHandler<T, N>
where
    T: EntityTrait + Send + Sync + 'static,
    N: EntityTrait + Send + Sync + 'static,
    Self: Send + Sync,
{
    async fn list_with_pagination(
        &self,
        page: u64,
        limit: u64,
        lang: i32,
    ) -> Result<PaginatedResource<WithName<T::Model>>>;

    async fn get_by_id(&self, id: i32, lang: i32) -> Result<WithName<T::Model>>;
}
