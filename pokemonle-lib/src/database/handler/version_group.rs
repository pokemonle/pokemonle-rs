use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use super::DatabaseConnection;
use crate::database::schema::version_groups::dsl::*;
use crate::model::VersionGroup;

pub struct VersionGroupHandler {
    pub connection: Pool<ConnectionManager<DatabaseConnection>>,
}

impl VersionGroupHandler {
    pub fn new(connection: Pool<ConnectionManager<DatabaseConnection>>) -> Self {
        VersionGroupHandler { connection }
    }

    pub fn get_all_version_groups(&self) -> Vec<VersionGroup> {
        version_groups
            .select(VersionGroup::as_select())
            .load(&mut self.connection.get().unwrap())
            .expect("Error loading version_groups")
    }

    pub fn get_version_group_by_id(&self, version_group_id: i32) -> Option<VersionGroup> {
        version_groups
            .filter(id.eq(version_group_id))
            .select(VersionGroup::as_select())
            .first::<VersionGroup>(&mut self.connection.get().unwrap())
            .ok()
    }
}
