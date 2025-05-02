use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use super::DatabaseConnection;
use crate::database::schema::abilities::dsl::*;
use crate::model::Ability;

pub struct AbilityHandler {
    pub connection: Pool<ConnectionManager<DatabaseConnection>>,
}

impl AbilityHandler {
    pub fn new(connection: Pool<ConnectionManager<DatabaseConnection>>) -> Self {
        AbilityHandler { connection }
    }

    pub fn get_all_abilities(&self) -> Vec<Ability> {
        abilities
            .select(Ability::as_select())
            .load(&mut self.connection.get().unwrap())
            .expect("Error loading abilities")
    }

    pub fn get_ability_by_id(&self, ability_id: i32) -> Option<Ability> {
        abilities
            .filter(id.eq(ability_id))
            .select(Ability::as_select())
            .first::<Ability>(&mut self.connection.get().unwrap())
            .ok()
    }
}
