use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use super::{DatabaseConnection, DatabaseHandler};
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

    fn get_all_resources(&self) -> Vec<Self::Resource> {
        types
            .select(Type::as_select())
            .load(&mut self.connection.get().unwrap())
            .expect("Error loading types")
    }

    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
        types
            .filter(id.eq(resource_id))
            .select(Type::as_select())
            .first::<Type>(&mut self.connection.get().unwrap())
            .ok()
    }
}
