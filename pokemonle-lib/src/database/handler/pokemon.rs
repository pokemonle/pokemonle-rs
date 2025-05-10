use crate::database::pagination::PaginatedResource;
use crate::database::schema::pokemon_species_names;
use crate::database::schema::{pokemon, pokemon_species};
use crate::model::{Pokemon, PokemonSpecies};
use crate::{impl_database_handler, impl_database_locale_handler};

impl_database_handler!(
    PokemonHandler,
    Pokemon,
    pokemon::dsl::pokemon,
    pokemon::dsl::id
);

impl PokemonHandler {
    pub fn get_all_identifiers(&self) -> Vec<String> {
        use crate::database::schema::pokemon::dsl::*;
        use diesel::prelude::*;
        pokemon
            .select(identifier)
            .load::<String>(&mut self.connection.get().unwrap())
            .expect("Error loading pokemon identifiers")
    }

    // get a random pokemon from given generation array
    pub fn get_random_pokemon(&self, generations: &[usize]) -> Option<PokemonSpecies> {
        use crate::database::schema::pokemon_species::dsl::*;
        use diesel::prelude::*;
        define_sql_function!(fn random() -> Text);

        let gens: Vec<i32> = generations.iter().map(|&x| x as i32).collect();

        pokemon_species
            .select(PokemonSpecies::as_select())
            .filter(generation_id.eq_any(gens))
            .order(random())
            .first::<PokemonSpecies>(&mut self.connection.get().unwrap())
            .ok()
    }
    // list pokemons from pokemon_abilities table with given ability_id
    pub fn list_by_ability(&self, _ability_id: i32) -> PaginatedResource<Pokemon> {
        use crate::database::schema::pokemon;
        use crate::database::schema::pokemon_abilities::dsl::*;
        use diesel::prelude::*;
        // select * from pokemon where id in (select pokemon_id from pokemon_abilities where ability_id = ability_id)
        let query = pokemon_abilities
            .select(pokemon_id)
            .filter(ability_id.eq(_ability_id));
        let pokemons = pokemon::table
            .filter(pokemon::id.eq_any(query))
            .load::<Pokemon>(&mut self.connection.get().unwrap())
            .expect("Error loading pokemons");
        PaginatedResource::new_from_vec(pokemons)
    }
}

impl_database_handler!(
    PokemonSpecieHandler,
    PokemonSpecies,
    pokemon_species::dsl::pokemon_species,
    pokemon_species::dsl::id
);

impl_database_locale_handler!(
    PokemonHandler,
    Pokemon,
    pokemon::dsl::pokemon,
    pokemon::dsl::id,
    pokemon_species_names::dsl::pokemon_species_names,
    pokemon_species_names::dsl::pokemon_species_id,
    pokemon_species_names::dsl::name,
    pokemon_species_names::dsl::local_language_id
);

impl_database_locale_handler!(
    PokemonSpecieHandler,
    PokemonSpecies,
    pokemon_species::dsl::pokemon_species,
    pokemon_species::dsl::id,
    pokemon_species_names::dsl::pokemon_species_names,
    pokemon_species_names::dsl::pokemon_species_id,
    pokemon_species_names::dsl::name,
    pokemon_species_names::dsl::local_language_id
);
