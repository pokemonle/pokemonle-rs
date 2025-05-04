use diesel::dsl::count;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use super::{DatabaseConnection, DatabaseHandler};
use crate::database::pagination::{Paginated, PaginatedResource};
use crate::database::schema::languages::dsl::*;
use crate::model::Language;

pub struct LanguageHandler {
    pub connection: Pool<ConnectionManager<DatabaseConnection>>,
}

impl LanguageHandler {
    pub fn new(connection: Pool<ConnectionManager<DatabaseConnection>>) -> Self {
        LanguageHandler { connection }
    }
}

impl DatabaseHandler for LanguageHandler {
    type Resource = Language;

    fn get_all_resources(&self, pagination: Paginated) -> PaginatedResource<Self::Resource> {
        let mut conn = self.connection.get().unwrap();

        let total_items = languages.select(count(id)).first::<i64>(&mut conn).unwrap();
        let total_pages = pagination.pages(total_items);

        let items = languages
            .select(Language::as_select())
            .limit(pagination.limit())
            .offset(pagination.offset())
            .load(&mut conn)
            .expect("Error loading languages");

        PaginatedResource {
            data: items,
            total_pages,
            total_items,
            page: pagination.page,
            per_page: pagination.per_page,
        }
    }

    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
        languages
            .filter(id.eq(resource_id))
            .select(Language::as_select())
            .first::<Language>(&mut self.connection.get().unwrap())
            .ok()
    }
}
