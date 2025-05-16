use crate::{prelude::*, types::WithName};
use async_trait::async_trait;
use sea_orm::{EntityTrait, FromQueryResult, PrimaryKeyTrait, Related};

use crate::types::response::PaginatedResource;

pub trait LocalizedEntity: EntityTrait {
    type NameEntity: EntityTrait + Related<Self>;

    fn get_identifier(model: &Self::Model) -> String;
    fn get_name(name_model: &<Self::NameEntity as EntityTrait>::Model) -> String;
    fn language_id_column() -> <Self::NameEntity as EntityTrait>::Column;
}

#[async_trait]
pub trait ResourceHandler<T>
where
    T: EntityTrait,
    T::PrimaryKey: PrimaryKeyTrait<ValueType = i32>,
    T::Model: FromQueryResult + Sized + Send + Sync,
    Self: Send,
{
    async fn list_with_pagination(
        &self,
        page: u64,
        limit: u64,
    ) -> Result<PaginatedResource<T::Model>>;

    async fn get_by_id(&self, id: i32) -> Result<T::Model>;
}

#[async_trait]
pub trait LocalizedResourceHandler<T>
where
    T: EntityTrait,
    T::PrimaryKey: PrimaryKeyTrait<ValueType = i32>,
    T::Model: FromQueryResult + Sized + Send + Sync,
    Self: Send,
{
    async fn list_with_pagination(
        &self,
        page: u64,
        limit: u64,
        lang: i32,
    ) -> Result<PaginatedResource<WithName<T::Model>>>;

    async fn get_by_id(&self, id: i32, lang: i32) -> Result<WithName<T::Model>>;
}
