use diesel::dsl::count;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use super::{DatabaseConnection, DatabaseHandler};
use crate::database::pagination::{Paginated, PaginatedResource};
use crate::database::schema::generations::dsl::*;
use crate::model::Generation;

pub struct GenerationHandler {
    pub connection: Pool<ConnectionManager<DatabaseConnection>>,
}

impl GenerationHandler {
    pub fn new(connection: Pool<ConnectionManager<DatabaseConnection>>) -> Self {
        GenerationHandler { connection }
    }
}

impl DatabaseHandler for GenerationHandler {
    type Resource = Generation;

    fn get_all_resources(&self, pagination: Paginated) -> PaginatedResource<Self::Resource> {
        let mut conn = self.connection.get().unwrap();

        let total_items = generations
            .select(count(id))
            .first::<i64>(&mut conn)
            .unwrap();
        let total_pages = pagination.pages(total_items);

        let items = generations
            .select(Generation::as_select())
            .limit(pagination.limit())
            .offset(pagination.offset())
            .load(&mut conn)
            .expect("Error loading generations");

        PaginatedResource {
            data: items,
            total_pages,
            total_items,
            page: pagination.page,
            per_page: pagination.per_page,
        }
    }

    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
        generations
            .filter(id.eq(resource_id))
            .select(Generation::as_select())
            .first::<Generation>(&mut self.connection.get().unwrap())
            .ok()
    }
}
