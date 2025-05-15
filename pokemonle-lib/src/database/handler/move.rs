use crate::database::schema::{move_flavor_text, move_names, moves};
use crate::model::{Languaged, Move, Pokemon};
use crate::prelude::*;
use crate::types::response::PaginatedResource;
use crate::{
    impl_database_flavor_text_handler, impl_database_handler, impl_database_locale_handler,
};

impl_database_handler!(MoveHandler, Move, moves::dsl::moves, moves::dsl::id);

impl_database_locale_handler!(
    MoveHandler,
    Move,
    moves::dsl::moves,
    moves::dsl::id,
    move_names::dsl::move_names,
    move_names::dsl::move_id,
    move_names::dsl::name,
    move_names::dsl::local_language_id
);

impl_database_flavor_text_handler!(
    MoveHandler,
    move_flavor_text::dsl::move_flavor_text,
    move_flavor_text::dsl::move_id,
    move_flavor_text::dsl::flavor_text,
    move_flavor_text::dsl::language_id,
    move_flavor_text::dsl::version_group_id
);

impl MoveHandler {
    pub fn get_move_pokemons(
        &self,
        _move_id: i32,
        _lang: i32,
        _version_group_id: i32,
    ) -> Result<PaginatedResource<Languaged<Pokemon>>> {
        use crate::database::schema::pokemon::dsl::*;
        use crate::database::schema::pokemon_moves;
        use crate::database::schema::pokemon_species_names;
        use diesel::prelude::*;

        let query = pokemon_moves::table
            .filter(pokemon_moves::move_id.eq(_move_id))
            .filter(pokemon_moves::version_group_id.eq(_version_group_id))
            .select(pokemon_moves::pokemon_id);

        pokemon
            .filter(id.eq_any(query))
            .inner_join(
                pokemon_species_names::table
                    .on(species_id.eq(pokemon_species_names::pokemon_species_id)),
            )
            .filter(pokemon_species_names::local_language_id.eq(_lang))
            .select((Pokemon::as_select(), pokemon_species_names::name))
            .load::<(Pokemon, String)>(&mut self.connection.get().map_err(Error::R2D2PoolError)?)
            .map_err(Error::DieselError)
            .map(|pokemons| {
                PaginatedResource::new_from_vec(
                    pokemons
                        .into_iter()
                        .map(Languaged::new_from_tuple)
                        .collect(),
                )
            })
    }
}
