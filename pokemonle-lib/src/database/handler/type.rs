use diesel::dsl::count;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use super::{DatabaseConnection, DatabaseHandler};
use crate::database::pagination::{Paginated, PaginatedResource};
use crate::database::schema::types::dsl::*;
use crate::model::Type;

pub struct TypeHandler {
    pub connection: Pool<ConnectionManager<DatabaseConnection>>,
}

impl TypeHandler {
    pub fn new(connection: Pool<ConnectionManager<DatabaseConnection>>) -> Self {
        TypeHandler { connection }
    }
}

impl DatabaseHandler for TypeHandler {
    type Resource = Type;

    fn get_all_resources(&self, pagination: Paginated) -> PaginatedResource<Self::Resource> {
        let mut conn = self.connection.get().unwrap();

        let total_items = types.select(count(id)).first::<i64>(&mut conn).unwrap();
        let total_pages = pagination.pages(total_items);

        let items = types
            .select(Type::as_select())
            .limit(pagination.limit())
            .offset(pagination.offset())
            .load(&mut conn)
            .expect("Error loading types");

        PaginatedResource {
            data: items,
            total_pages,
            total_items,
            page: pagination.page,
            per_page: pagination.per_page,
        }
    }

    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
        types
            .filter(id.eq(resource_id))
            .select(Type::as_select())
            .first::<Type>(&mut self.connection.get().unwrap())
            .ok()
    }
}
