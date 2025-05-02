use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use super::DatabaseConnection;
use crate::database::schema::versions::dsl::*;
use crate::model::Version;

pub struct VersionHandler {
    pub connection: Pool<ConnectionManager<DatabaseConnection>>,
}

impl VersionHandler {
    pub fn new(connection: Pool<ConnectionManager<DatabaseConnection>>) -> Self {
        VersionHandler { connection }
    }

    pub fn get_all_versions(&self) -> Vec<Version> {
        versions
            .select(Version::as_select())
            .load(&mut self.connection.get().unwrap())
            .expect("Error loading versions")
    }

    pub fn get_version_by_id(&self, version_id: i32) -> Option<Version> {
        versions
            .filter(id.eq(version_id))
            .select(Version::as_select())
            .first::<Version>(&mut self.connection.get().unwrap())
            .ok()
    }
}
