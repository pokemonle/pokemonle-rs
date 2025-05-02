mod ability;
mod language;
mod version;
mod version_group;

use crate::config::Config;
use crate::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{Connection, MultiConnection, PgConnection, QueryResult, SqliteConnection};

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
