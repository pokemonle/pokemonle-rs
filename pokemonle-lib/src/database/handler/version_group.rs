use diesel::dsl::count;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use super::{DatabaseConnection, DatabaseHandler};
use crate::database::pagination::{Paginated, PaginatedResource};
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

    fn get_all_resources(&self, pagination: Paginated) -> PaginatedResource<Self::Resource> {
        let mut conn = self.connection.get().unwrap();

        let total_items = version_groups
            .select(count(id))
            .first::<i64>(&mut conn)
            .unwrap();
        let total_pages = pagination.pages(total_items);

        let items = version_groups
            .select(VersionGroup::as_select())
            .limit(pagination.limit())
            .offset(pagination.offset())
            .load(&mut conn)
            .expect("Error loading version_groups");

        PaginatedResource {
            data: items,
            total_pages,
            total_items,
            page: pagination.page,
            per_page: pagination.per_page,
        }
    }

    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
        version_groups
            .filter(id.eq(resource_id))
            .select(VersionGroup::as_select())
            .first::<VersionGroup>(&mut self.connection.get().unwrap())
            .ok()
    }
}
