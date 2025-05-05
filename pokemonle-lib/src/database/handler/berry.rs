use diesel::dsl::count;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use super::{DatabaseConnection, DatabaseHandler};
use crate::database::pagination::{Paginated, PaginatedResource};
use crate::database::schema::berries::dsl::*;
use crate::model::{Berry, BerryFirmness};

pub struct BerryHandler {
    pub connection: Pool<ConnectionManager<DatabaseConnection>>,
}

impl BerryHandler {
    pub fn new(connection: Pool<ConnectionManager<DatabaseConnection>>) -> Self {
        BerryHandler { connection }
    }

    pub fn firmness(&self) -> BerryFirmnessHandler {
        BerryFirmnessHandler {
            connection: self.connection.clone(),
        }
    }
}

impl DatabaseHandler for BerryHandler {
    type Resource = Berry;

    fn get_all_resources(&self, pagination: Paginated) -> PaginatedResource<Self::Resource> {
        let mut conn = self.connection.get().unwrap();

        let total_items = berries.select(count(id)).first::<i64>(&mut conn).unwrap();
        let total_pages = pagination.pages(total_items);

        let items = berries
            .select(Berry::as_select())
            .limit(pagination.limit())
            .offset(pagination.offset())
            .load(&mut conn)
            .expect("Error loading berries");

        PaginatedResource {
            data: items,
            total_pages,
            total_items,
            page: pagination.page,
            per_page: pagination.per_page,
        }
    }

    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
        berries
            .filter(id.eq(resource_id))
            .select(Berry::as_select())
            .first::<Berry>(&mut self.connection.get().unwrap())
            .ok()
    }
}

pub struct BerryFirmnessHandler {
    pub connection: Pool<ConnectionManager<DatabaseConnection>>,
}

impl DatabaseHandler for BerryFirmnessHandler {
    type Resource = BerryFirmness;

    fn get_all_resources(&self, pagination: Paginated) -> PaginatedResource<Self::Resource> {
        use crate::database::schema::berry_firmness::dsl::*;
        let mut conn = self.connection.get().unwrap();

        let total_items = berry_firmness
            .select(count(id))
            .first::<i64>(&mut conn)
            .unwrap();
        let total_pages = pagination.pages(total_items);

        let items = berry_firmness
            .select(BerryFirmness::as_select())
            .limit(pagination.limit())
            .offset(pagination.offset())
            .load(&mut conn)
            .expect("Error loading berry firmness");

        PaginatedResource {
            data: items,
            total_pages,
            total_items,
            page: pagination.page,
            per_page: pagination.per_page,
        }
    }

    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
        use crate::database::schema::berry_firmness::dsl::*;
        berry_firmness
            .filter(id.eq(resource_id))
            .select(BerryFirmness::as_select())
            .first::<BerryFirmness>(&mut self.connection.get().unwrap())
            .ok()
    }
}
