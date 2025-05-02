use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use super::{DatabaseConnection, DatabaseHandler};
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

    fn get_all_resources(&self) -> Vec<Self::Resource> {
        generations
            .select(Generation::as_select())
            .load(&mut self.connection.get().unwrap())
            .expect("Error loading generations")
    }

    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
        generations
            .filter(id.eq(resource_id))
            .select(Generation::as_select())
            .first::<Generation>(&mut self.connection.get().unwrap())
            .ok()
    }
}
