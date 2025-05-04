use super::{DatabaseConnection, DatabaseHandler};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use pokemonle_trait::StructName;

use crate::database::schema::{pokemon_colors, pokemon_habitats, pokemon_shapes};
use crate::model::{
    Pokemon, PokemonColor, PokemonHabitat, PokemonShape, PokemonSpecieDetail, PokemonSpecies,
};

pub struct PokemonHandler {
    pub connection: Pool<ConnectionManager<DatabaseConnection>>,
}

impl PokemonHandler {
    pub fn new(connection: Pool<ConnectionManager<DatabaseConnection>>) -> Self {
        PokemonHandler { connection }
    }

    pub fn specie_handler(&self) -> PokemonSpecieHandler {
        PokemonSpecieHandler::new(self.connection.clone())
    }

    // get a random pokemon from given generation array
    pub fn get_random_pokemon(&self, generations: &[usize]) -> Option<PokemonSpecies> {
        use crate::database::schema::pokemon_species::dsl::*;
        define_sql_function!(fn random() -> Text);

        let gens: Vec<i32> = generations.iter().map(|&x| x as i32).collect();

        pokemon_species
            .select(PokemonSpecies::as_select())
            .filter(generation_id.eq_any(gens))
            .order(random())
            .first::<PokemonSpecies>(&mut self.connection.get().unwrap())
            .ok()
    }
}

impl DatabaseHandler for PokemonHandler {
    type Resource = Pokemon;

    fn get_all_resources(&self) -> Vec<Self::Resource> {
        use crate::database::schema::pokemon::dsl::*;
        pokemon
            .select(Pokemon::as_select())
            .load(&mut self.connection.get().unwrap())
            .expect("Error loading pokemons")
    }

    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
        use crate::database::schema::pokemon::dsl::*;
        pokemon
            .filter(id.eq(resource_id))
            .select(Pokemon::as_select())
            .first::<Pokemon>(&mut self.connection.get().unwrap())
            .ok()
    }
}

pub struct PokemonSpecieHandler {
    pub connection: Pool<ConnectionManager<DatabaseConnection>>,
}

impl PokemonSpecieHandler {
    pub fn new(connection: Pool<ConnectionManager<DatabaseConnection>>) -> Self {
        PokemonSpecieHandler { connection }
    }
}

impl DatabaseHandler for PokemonSpecieHandler {
    type Resource = PokemonSpecieDetail;

    fn get_all_resources(&self) -> Vec<Self::Resource> {
        use crate::database::schema::pokemon_species::dsl::*;

        pokemon_species
            .inner_join(pokemon_colors::table)
            .inner_join(pokemon_shapes::table)
            // .inner_join(pokemon_habitats::table)
            .select((
                PokemonSpecies::as_select(),
                PokemonColor::as_select(),
                PokemonShape::as_select(),
                // PokemonHabitat::as_select(),
            ))
            .load(&mut self.connection.get().unwrap())
            .expect("Error loading pokemon species")
    }

    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
        use crate::database::schema::pokemon_species::dsl::*;
        pokemon_species
            .inner_join(pokemon_colors::table)
            .inner_join(pokemon_shapes::table)
            // .inner_join(pokemon_habitats::table)
            .filter(id.eq(resource_id))
            .select((
                PokemonSpecies::as_select(),
                PokemonColor::as_select(),
                PokemonShape::as_select(),
                // PokemonHabitat::as_select(),
            ))
            .first(&mut self.connection.get().unwrap())
            .ok()
    }
}
