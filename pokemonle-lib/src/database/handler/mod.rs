mod ability;
mod language;
mod version;
mod version_group;

use crate::config::Config;
use crate::prelude::*;
use diesel::migration::{MigrationVersion, Result as MigrationResult};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{Connection, MultiConnection, PgConnection, QueryResult, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::sync::{Mutex, Once};
use tracing::debug;

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
    fn get_all_resources(&self) -> Vec<Self::Resource>;
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

    pub fn language_handler(&self) -> language::LanguageHandler {
        language::LanguageHandler::new(self.connection.clone())
    }

    pub fn version_handler(&self) -> version::VersionHandler {
        version::VersionHandler::new(self.connection.clone())
    }

    pub fn version_group_handler(&self) -> version_group::VersionGroupHandler {
        version_group::VersionGroupHandler::new(self.connection.clone())
    }

    pub fn ability_handler(&self) -> ability::AbilityHandler {
        ability::AbilityHandler::new(self.connection.clone())
    }
}
