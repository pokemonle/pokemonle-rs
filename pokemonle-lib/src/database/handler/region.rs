use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use super::{DatabaseConnection, DatabaseHandler};
use crate::database::schema::versions::dsl::*;
use crate::model::Version;

pub struct VersionHandler {
    pub connection: Pool<ConnectionManager<DatabaseConnection>>,
}

impl VersionHandler {
    pub fn new(connection: Pool<ConnectionManager<DatabaseConnection>>) -> Self {
        VersionHandler { connection }
    }
}

impl DatabaseHandler for VersionHandler {
    type Resource = Version;

    fn get_all_resources(&self) -> Vec<Self::Resource> {
        versions
            .select(Version::as_select())
            .load(&mut self.connection.get().unwrap())
            .expect("Error loading versions")
    }

    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
        versions
            .filter(id.eq(resource_id))
            .select(Version::as_select())
            .first::<Version>(&mut self.connection.get().unwrap())
            .ok()
    }
}
