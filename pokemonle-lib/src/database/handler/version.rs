use diesel::dsl::count;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use super::{DatabaseConnection, DatabaseHandler};
use crate::database::pagination::{Paginated, PaginatedResource};
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

    fn get_all_resources(&self, pagination: Paginated) -> PaginatedResource<Self::Resource> {
        let mut conn = self.connection.get().unwrap();

        let total_items = versions.select(count(id)).first::<i64>(&mut conn).unwrap();
        let total_pages = pagination.pages(total_items);

        let items = versions
            .select(Version::as_select())
            .limit(pagination.limit())
            .offset(pagination.offset())
            .load(&mut conn)
            .expect("Error loading versions");

        PaginatedResource {
            data: items,
            total_pages,
            total_items,
            page: pagination.page,
            per_page: pagination.per_page,
        }
    }

    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
        versions
            .filter(id.eq(resource_id))
            .select(Version::as_select())
            .first::<Version>(&mut self.connection.get().unwrap())
            .ok()
    }
}
