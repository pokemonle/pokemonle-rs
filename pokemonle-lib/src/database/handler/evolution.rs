use crate::database::pagination::PaginatedResource;
use crate::database::schema::{evolution_chains, evolution_triggers};
use crate::impl_database_handler;
use crate::model::{EvolutionChain, EvolutionTrigger, Languaged, PokemonSpecies};
use crate::prelude::*;

impl_database_handler!(
    EvolutionChainHandler,
    EvolutionChain,
    evolution_chains::dsl::evolution_chains,
    evolution_chains::dsl::id
);

impl_database_handler!(
    EvolutionTriggerHandler,
    EvolutionTrigger,
    evolution_triggers::dsl::evolution_triggers,
    evolution_triggers::dsl::id
);

impl EvolutionChainHandler {
    pub fn get_pokemon_species_by_evolution_chain_id(
        &self,
        _evolution_chain_id: i32,
        locale_id: i32,
    ) -> Result<PaginatedResource<Languaged<PokemonSpecies>>> {
        use crate::database::schema::pokemon_species::dsl::*;
        use crate::database::schema::pokemon_species_names;
        use diesel::prelude::*;

        pokemon_species
            .inner_join(pokemon_species_names::table)
            .filter(evolution_chain_id.eq(Some(_evolution_chain_id)))
            .filter(pokemon_species_names::local_language_id.eq(locale_id))
            .select((PokemonSpecies::as_select(), pokemon_species_names::name))
            .load::<(PokemonSpecies, String)>(
                &mut self.connection.get().map_err(Error::R2D2PoolError)?,
            )
            .map_err(Error::DieselError)
            .map(|pokemons| {
                PaginatedResource::new_from_vec(
                    pokemons
                        .into_iter()
                        .map(|(p, n)| Languaged { item: p, name: n })
                        .collect(),
                )
            })
    }
}
