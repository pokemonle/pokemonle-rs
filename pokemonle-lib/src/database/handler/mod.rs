mod berry;
mod contest;
mod item;
mod pokemon;

use crate::config::Config;
use crate::{impl_handlers, prelude::*};
use diesel::migration::{MigrationVersion, Result as MigrationResult};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{Connection, MultiConnection, PgConnection, QueryResult, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::sync::{Mutex, Once};
use tracing::debug;

use super::pagination::PaginatedResource;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

static VFS: Mutex<(i32, Once)> = Mutex::new((0, Once::new()));

#[derive(MultiConnection)]
pub enum DatabaseConnection {
    Pg(PgConnection),
    Sqlite(SqliteConnection),
}

pub struct DatabaseClient {
    pub connection: DatabaseConnection,
}

impl DatabaseClient {
    pub fn new() -> Result<Self> {
        let config = Config::new()?;
        let connection = {
            let url = &config.database_url;
            if url.starts_with("postgres://") {
                DatabaseConnection::Pg(PgConnection::establish(url)?)
            } else {
                DatabaseConnection::Sqlite(SqliteConnection::establish(url)?)
            }
        };

        Ok(DatabaseClient { connection })
    }
}

pub trait DatabaseHandler {
    type Resource;
    fn get_all_resources(
        &self,
        pagination: super::pagination::Paginated,
    ) -> PaginatedResource<Self::Resource>;
    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource>;
}

#[derive(Clone)]
pub struct DatabaseClientPooled {
    connection: Pool<ConnectionManager<DatabaseConnection>>,
}

impl DatabaseClientPooled {
    pub fn new() -> Result<Self> {
        let config = Config::new()?;

        let pool = if config.database_url.starts_with("postgres://") {
            Pool::builder().build(ConnectionManager::new(config.database_url))?
        } else {
            let (vfs, once) = &*VFS.lock().unwrap();
            let url = match vfs {
                0 => &config.database_url,
                1 => &format!("file:{}?vfs=opfs-sahpool", config.database_url),
                2 => &format!("file:{}?vfs=relaxed-idb", config.database_url),
                _ => unreachable!(),
            };

            let mut conn = SqliteConnection::establish(url)
                .unwrap_or_else(|_| panic!("{}", format!("Error connecting to {}", url)));
            once.call_once(|| {
                // Run migrations
                let r = conn
                    .pending_migrations(MIGRATIONS)
                    .expect("Error loading migrations")
                    .iter()
                    .map(|m| {
                        debug!("Running migration: {}", m.name());
                        conn.run_migration(m)
                    })
                    .collect::<MigrationResult<Vec<MigrationVersion>>>();

                r.unwrap();
            });

            Pool::builder().build(ConnectionManager::new(config.database_url))?
        };

        Ok(DatabaseClientPooled { connection: pool })
    }
}

impl_handlers! {
    ability: ability::AbilityHandler,
    berry: berry::BerryHandler,
    berry_firmness: berry::BerryFirmnessHandler,
    contest_effect: contest::ContestEffectHandler,
    contest_type: contest::ContestTypeHandler,
    generation: generation::GenerationHandler,
    item: item::ItemHandler,
    item_category: item::ItemCategoryHandler,
    item_pocket: item::ItemPocketHandler,
    language: language::LanguageHandler,
    pokemon: pokemon::PokemonHandler,
    pokemon_specie: pokemon::PokemonSpecieHandler,
    r#type: r#type::TypeHandler,
    version: version::VersionHandler,
    version_group: version_group::VersionGroupHandler,
}

mod ability {
    use crate::impl_database_handler;
    use crate::model::Ability;

    use crate::database::schema::abilities;
    impl_database_handler!(
        AbilityHandler,
        Ability,
        abilities::dsl::abilities,
        abilities::dsl::id
    );
}

mod generation {
    use crate::database::schema::generations;
    use crate::impl_database_handler;
    use crate::model::Generation;

    impl_database_handler!(
        GenerationHandler,
        Generation,
        generations::dsl::generations,
        generations::dsl::id
    );
}

mod language {
    use crate::database::schema::languages;
    use crate::impl_database_handler;
    use crate::model::Language;

    impl_database_handler!(
        LanguageHandler,
        Language,
        languages::dsl::languages,
        languages::dsl::id
    );
}

mod r#type {
    use crate::database::schema::types;
    use crate::impl_database_handler;
    use crate::model::Type;

    impl_database_handler!(TypeHandler, Type, types::dsl::types, types::dsl::id);
}

mod version {
    use crate::database::schema::versions;
    use crate::impl_database_handler;
    use crate::model::Version;

    impl_database_handler!(
        VersionHandler,
        Version,
        versions::dsl::versions,
        versions::dsl::id
    );
}

mod version_group {
    use crate::database::schema::version_groups;
    use crate::impl_database_handler;
    use crate::model::VersionGroup;

    impl_database_handler!(
        VersionGroupHandler,
        VersionGroup,
        version_groups::dsl::version_groups,
        version_groups::dsl::id
    );
}
