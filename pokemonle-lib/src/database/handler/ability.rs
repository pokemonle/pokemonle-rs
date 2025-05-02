use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use super::{DatabaseConnection, DatabaseHandler};
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

    fn get_all_resources(&self) -> Vec<Self::Resource> {
        abilities
            .select(Ability::as_select())
            .load(&mut self.connection.get().unwrap())
            .expect("Error loading abilities")
    }

    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
        abilities
            .filter(id.eq(resource_id))
            .select(Ability::as_select())
            .first::<Ability>(&mut self.connection.get().unwrap())
            .ok()
    }
}
