use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use super::{DatabaseConnection, DatabaseHandler};
use crate::database::schema::pokemon::dsl::*;
use crate::model::Pokemon;

pub struct PokemonHandler {
    pub connection: Pool<ConnectionManager<DatabaseConnection>>,
}

impl PokemonHandler {
    pub fn new(connection: Pool<ConnectionManager<DatabaseConnection>>) -> Self {
        PokemonHandler { connection }
    }
}

impl DatabaseHandler for PokemonHandler {
    type Resource = Pokemon;

    fn get_all_resources(&self) -> Vec<Self::Resource> {
        pokemon
            .select(Pokemon::as_select())
            .load(&mut self.connection.get().unwrap())
            .expect("Error loading pokemons")
    }

    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
        pokemon
            .filter(id.eq(resource_id))
            .select(Pokemon::as_select())
            .first::<Pokemon>(&mut self.connection.get().unwrap())
            .ok()
    }
}
