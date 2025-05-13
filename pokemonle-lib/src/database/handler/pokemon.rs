use crate::database::pagination::PaginatedResource;
use crate::database::schema::{pokemon, pokemon_species};
use crate::database::schema::{pokemon_species_flavor_text, pokemon_species_names};
use crate::model::{Ability, Languaged, Pokemon, PokemonAbility, PokemonSpecies, WithSlot};
use crate::{
    impl_database_flavor_text_handler, impl_database_handler, impl_database_locale_handler,
};

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
    pub fn list_by_ability(
        &self,
        _ability_id: i32,
        _lang: i32,
    ) -> PaginatedResource<Languaged<Pokemon>> {
        use crate::database::schema::pokemon;
        use crate::database::schema::pokemon_abilities::dsl::*;
        use crate::database::schema::pokemon_species_names;
        use diesel::prelude::*;
        // select * from pokemon where id in (select pokemon_id from pokemon_abilities where ability_id = ability_id)
        let query = pokemon_abilities
            .select(pokemon_id)
            .filter(ability_id.eq(_ability_id));
        let pokemons = pokemon::table
            .filter(pokemon::id.eq_any(query))
            .inner_join(
                pokemon_species_names::table
                    .on(pokemon::id.eq(pokemon_species_names::pokemon_species_id)),
            )
            .filter(pokemon_species_names::local_language_id.eq(_lang))
            .select((Pokemon::as_select(), pokemon_species_names::name))
            .load::<(Pokemon, String)>(&mut self.connection.get().unwrap())
            .expect("Error loading pokemons");
        PaginatedResource::new_from_vec(
            pokemons
                .into_iter()
                .map(|(p, n)| Languaged { item: p, name: n })
                .collect(),
        )
    }

    pub fn get_pokemon_abilities(
        &self,
        _pokemon_id: i32,
        _lang: i32,
    ) -> PaginatedResource<Languaged<WithSlot<Ability>>> {
        use crate::database::schema::abilities;
        use crate::database::schema::ability_names;
        use crate::database::schema::pokemon_abilities::dsl::*;
        use diesel::prelude::*;

        let items = pokemon_abilities
            .inner_join(abilities::table.on(ability_id.eq(abilities::id)))
            .inner_join(ability_names::table.on(abilities::id.eq(ability_names::ability_id)))
            .filter(pokemon_id.eq(_pokemon_id))
            .filter(ability_names::local_language_id.eq(_lang))
            .select((
                Ability::as_select(),
                PokemonAbility::as_select(),
                ability_names::name,
            ))
            .load::<(Ability, PokemonAbility, String)>(&mut self.connection.get().unwrap())
            .expect("Error loading pokemon abilities");

        PaginatedResource::new_from_vec(
            items
                .into_iter()
                .map(|(a, p, n)| Languaged {
                    item: WithSlot {
                        item: a,
                        slot: p.slot,
                        is_hidden: p.is_hidden,
                    },
                    name: n,
                })
                .collect(),
        )
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

impl_database_flavor_text_handler!(
    PokemonSpecieHandler,
    pokemon_species_flavor_text::dsl::pokemon_species_flavor_text,
    pokemon_species_flavor_text::dsl::species_id,
    pokemon_species_flavor_text::dsl::flavor_text,
    pokemon_species_flavor_text::dsl::language_id,
    pokemon_species_flavor_text::dsl::version_id
);
