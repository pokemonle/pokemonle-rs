use diesel::dsl::count;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use super::{DatabaseConnection, DatabaseHandler};
use crate::database::pagination::{Paginated, PaginatedResource};
use crate::database::schema::abilities::dsl::*;
use crate::model::Ability;

pub struct AbilityHandler {
    pub connection: Pool<ConnectionManager<DatabaseConnection>>,
}

impl AbilityHandler {
    pub fn new(connection: Pool<ConnectionManager<DatabaseConnection>>) -> Self {
        AbilityHandler { connection }
    }
}

impl DatabaseHandler for AbilityHandler {
    type Resource = Ability;

    fn get_all_resources(&self, pagination: Paginated) -> PaginatedResource<Self::Resource> {
        let mut conn = self.connection.get().unwrap();

        let total_items = abilities.select(count(id)).first::<i64>(&mut conn).unwrap();
        let total_pages = pagination.pages(total_items);

        let items = abilities
            .select(Ability::as_select())
            .limit(pagination.limit())
            .offset(pagination.offset())
            .load(&mut conn)
            .expect("Error loading abilities");

        PaginatedResource {
            data: items,
            total_pages,
            total_items,
            page: pagination.page,
            per_page: pagination.per_page,
        }
    }

    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
        abilities
            .filter(id.eq(resource_id))
            .select(Ability::as_select())
            .first::<Ability>(&mut self.connection.get().unwrap())
            .ok()
    }
}
