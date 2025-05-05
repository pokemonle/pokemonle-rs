use diesel::dsl::count;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use super::{DatabaseConnection, DatabaseHandler};
use crate::database::pagination::{Paginated, PaginatedResource};
use crate::database::schema::contest_effects::dsl::*;
use crate::database::schema::contest_types::dsl::*;
use crate::model::{ContestEffect, ContestType};

pub struct ContestTypeHandler {
    pub connection: Pool<ConnectionManager<DatabaseConnection>>,
}

impl ContestTypeHandler {
    pub fn new(connection: Pool<ConnectionManager<DatabaseConnection>>) -> Self {
        ContestTypeHandler { connection }
    }
}

impl DatabaseHandler for ContestTypeHandler {
    type Resource = ContestType;

    fn get_all_resources(&self, pagination: Paginated) -> PaginatedResource<Self::Resource> {
        use crate::database::schema::contest_types::dsl::*;
        let mut conn = self.connection.get().unwrap();

        let total_items = contest_types
            .select(count(id))
            .first::<i64>(&mut conn)
            .unwrap();
        let total_pages = pagination.pages(total_items);

        let items = contest_types
            .select(ContestType::as_select())
            .limit(pagination.limit())
            .offset(pagination.offset())
            .load(&mut conn)
            .expect("Error loading contest types");

        PaginatedResource {
            data: items,
            total_pages,
            total_items,
            page: pagination.page,
            per_page: pagination.per_page,
        }
    }

    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
        use crate::database::schema::contest_types::dsl::*;
        contest_types
            .filter(id.eq(resource_id))
            .select(ContestType::as_select())
            .first::<ContestType>(&mut self.connection.get().unwrap())
            .ok()
    }
}

pub struct ContestEffectHandler {
    pub connection: Pool<ConnectionManager<DatabaseConnection>>,
}

impl ContestEffectHandler {
    pub fn new(connection: Pool<ConnectionManager<DatabaseConnection>>) -> Self {
        ContestEffectHandler { connection }
    }
}

impl DatabaseHandler for ContestEffectHandler {
    type Resource = ContestEffect;

    fn get_all_resources(&self, pagination: Paginated) -> PaginatedResource<Self::Resource> {
        use crate::database::schema::contest_effects::dsl::*;
        let mut conn = self.connection.get().unwrap();

        let total_items = contest_effects
            .select(count(id))
            .first::<i64>(&mut conn)
            .unwrap();
        let total_pages = pagination.pages(total_items);

        let items = contest_effects
            .select(ContestEffect::as_select())
            .limit(pagination.limit())
            .offset(pagination.offset())
            .load(&mut conn)
            .expect("Error loading contest effects");

        PaginatedResource {
            data: items,
            total_pages,
            total_items,
            page: pagination.page,
            per_page: pagination.per_page,
        }
    }

    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
        use crate::database::schema::contest_effects::dsl::*;
        contest_effects
            .filter(id.eq(resource_id))
            .select(ContestEffect::as_select())
            .first::<ContestEffect>(&mut self.connection.get().unwrap())
            .ok()
    }
}
