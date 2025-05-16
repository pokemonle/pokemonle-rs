pub mod entry;
pub mod pokemon;
pub mod r#trait;
use crate::{
    prelude::*,
    types::{response::PaginatedResource, WithName},
};
use async_trait::async_trait;
use r#trait::{LocalizedEntity, LocalizedResourceHandler, ResourceHandler};
use sea_orm::{Database, DbConn, EntityTrait, FromQueryResult, PaginatorTrait, PrimaryKeyTrait};

use crate::config::Config;

#[derive(Clone, Debug)]
pub struct DatabaseClient {
    conn: DbConn,
}

impl DatabaseClient {
    pub async fn new() -> Result<Self> {
        let config = Config::new()?;

        let db = Database::connect(config.database_url).await?;

        Ok(Self { conn: db })
    }
}

#[async_trait]
impl<T> ResourceHandler<T> for DatabaseClient
where
    T: EntityTrait,
    T::PrimaryKey: PrimaryKeyTrait<ValueType = i32>,
    T::Model: FromQueryResult + Sized + Send + Sync,
{
    async fn list_with_pagination(
        &self,
        page: u64,
        limit: u64,
    ) -> Result<PaginatedResource<T::Model>> {
        let paginator = T::find().paginate(&self.conn, limit);

        let data = paginator.fetch_page(page).await?;

        Ok(PaginatedResource {
            data,
            page,
            per_page: limit,
            total_pages: paginator.num_pages().await?,
            total_items: paginator.num_items().await?,
        })
    }

    async fn get_by_id(&self, id: i32) -> Result<<T as EntityTrait>::Model> {
        T::find_by_id(id)
            .one(&self.conn)
            .await
            .map_err(Error::ConnectionError)?
            .ok_or(Error::ResourceNotFound(format!(
                "resource {} not found",
                id
            )))
    }
}

#[async_trait]
impl<T> LocalizedResourceHandler<T> for DatabaseClient
where
    T: LocalizedEntity,
    T::PrimaryKey: PrimaryKeyTrait<ValueType = i32>,
    T::Model: FromQueryResult + Sized + Send + Sync,
{
    async fn list_with_pagination(
        &self,
        page: u64,
        limit: u64,
        lang: i32,
    ) -> Result<PaginatedResource<WithName<T::Model>>> {
        todo!()
    }

    async fn get_by_id(&self, id: i32, lang: i32) -> Result<WithName<T::Model>> {
        todo!()
    }
}
