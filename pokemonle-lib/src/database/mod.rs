pub mod ability;
pub mod entity;
pub mod evo;
pub mod language;
pub mod r#move;
pub mod r#trait;
pub mod version;
use crate::localized_resource_handler;
use crate::{prelude::*, types::response::PaginatedResource};
use async_trait::async_trait;
use entity::prelude::*;
use r#trait::{DBConnection, ResourceHandler};
use sea_orm::{
    ConnectOptions, Database, DbConn, DbErr, EntityTrait, FromQueryResult, PaginatorTrait,
    PrimaryKeyTrait,
};

use tracing::{debug, log::LevelFilter};

use crate::config::Config;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../migrations");

#[derive(Clone, Debug)]
pub struct DatabaseClient {
    pub conn: DbConn,
}

impl DatabaseClient {
    pub async fn new() -> Result<Self> {
        let config = Config::new()?;

        {
            debug!("Start database migration");
            let db = Database::connect(
                ConnectOptions::from(config.database_url.as_str())
                    .sqlx_logging(false)
                    .to_owned(),
            )
            .await?;
            MIGRATOR
                .run(db.get_sqlite_connection_pool())
                .await
                .map_err(|e| Error::ConnectionError(DbErr::Migration(e.to_string())))?;
            debug!("Database migration completed");
        }

        let connoection_options = Into::<ConnectOptions>::into(config.database_url)
            .sqlx_logging_level(LevelFilter::Off)
            .to_owned();

        let db = Database::connect(connoection_options).await?;

        Ok(Self { conn: db })
    }
}

#[async_trait]
impl DBConnection for DatabaseClient {
    async fn get_conn(self) -> DbConn {
        self.conn.clone()
    }
}

#[async_trait]
impl<T, PK> ResourceHandler<T> for DatabaseClient
where
    T: EntityTrait<PrimaryKey = PK>,
    PK: PrimaryKeyTrait<ValueType = i32>,
    T::Model: FromQueryResult + Sized + Send + Sync,
{
    async fn list_with_pagination(
        &self,
        page: u64,
        limit: u64,
    ) -> Result<PaginatedResource<T::Model>> {
        let paginator = T::find().paginate(&self.conn, limit);

        let data = paginator.fetch_page(page).await?;

        let total = paginator.num_items_and_pages().await?;

        Ok(PaginatedResource {
            data,
            page,
            per_page: limit,
            total_pages: total.number_of_pages,
            total_items: total.number_of_items,
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

localized_resource_handler!(
    PokemonSpecies,
    PokemonSpeciesNames,
    entity::pokemon_species_names::Column::LocalLanguageId,
    entity::pokemon_species_names::Column::Name
);
localized_resource_handler!(
    BerryFirmness,
    BerryFirmnessNames,
    entity::berry_firmness_names::Column::LocalLanguageId,
    entity::berry_firmness_names::Column::Name
);
localized_resource_handler!(
    Generations,
    GenerationNames,
    entity::generation_names::Column::LocalLanguageId,
    entity::generation_names::Column::Name
);

localized_resource_handler!(
    ItemPockets,
    ItemPocketNames,
    entity::item_pocket_names::Column::LocalLanguageId,
    entity::item_pocket_names::Column::Name
);
localized_resource_handler!(
    Items,
    ItemNames,
    entity::item_names::Column::LocalLanguageId,
    entity::item_names::Column::Name
);

localized_resource_handler!(
    Regions,
    RegionNames,
    entity::region_names::Column::LocalLanguageId,
    entity::region_names::Column::Name
);

localized_resource_handler!(
    Locations,
    LocationNames,
    entity::location_names::Column::LocalLanguageId,
    entity::location_names::Column::Name
);
