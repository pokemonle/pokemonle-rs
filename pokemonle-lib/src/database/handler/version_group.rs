use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use super::{DatabaseConnection, DatabaseHandler};
use crate::database::schema::version_groups::dsl::*;
use crate::model::VersionGroup;

pub struct VersionGroupHandler {
    pub connection: Pool<ConnectionManager<DatabaseConnection>>,
}

impl VersionGroupHandler {
    pub fn new(connection: Pool<ConnectionManager<DatabaseConnection>>) -> Self {
        VersionGroupHandler { connection }
    }
}

impl DatabaseHandler for VersionGroupHandler {
    type Resource = VersionGroup;

    fn get_all_resources(&self) -> Vec<Self::Resource> {
        version_groups
            .select(VersionGroup::as_select())
            .load(&mut self.connection.get().unwrap())
            .expect("Error loading version_groups")
    }

    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
        version_groups
            .filter(id.eq(resource_id))
            .select(VersionGroup::as_select())
            .first::<VersionGroup>(&mut self.connection.get().unwrap())
            .ok()
    }
}
