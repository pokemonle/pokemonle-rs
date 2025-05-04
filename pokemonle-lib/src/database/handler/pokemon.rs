use super::{DatabaseConnection, DatabaseHandler};
use diesel::dsl::count;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::database::pagination::{Paginated, PaginatedResource};
use crate::database::schema::{egg_groups, pokemon_colors, pokemon_shapes};
use crate::model::{
    EggGroup, Pokemon, PokemonColor, PokemonEggGroup, PokemonShape, PokemonSpecieDetail,
    PokemonSpecies,
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

    fn get_all_resources(&self, pagination: Paginated) -> PaginatedResource<Self::Resource> {
        use crate::database::schema::pokemon::dsl::*;

        let mut conn = self.connection.get().unwrap();

        let total_items = pokemon.select(count(id)).first::<i64>(&mut conn).unwrap();
        let total_pages = pagination.pages(total_items);

        let items = pokemon
            .select(Pokemon::as_select())
            .limit(pagination.limit())
            .offset(pagination.offset())
            .load(&mut conn)
            .expect("Error loading abilities");

        PaginatedResource {
            data: items,
            total_pages,
            total_items,
            page: pagination.page,
            per_page: pagination.per_page,
        }
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

    fn get_all_resources(&self, pagination: Paginated) -> PaginatedResource<Self::Resource> {
        use crate::database::schema::pokemon_species::dsl::*;

        let mut conn = self.connection.get().unwrap();
        let total_items = pokemon_species
            .select(count(id))
            .first::<i64>(&mut conn)
            .unwrap();
        let total_pages = pagination.pages(total_items);

        let all_species_with_details = pokemon_species
            .inner_join(pokemon_colors::table)
            .inner_join(pokemon_shapes::table)
            // .inner_join(pokemon_habitats::table) // 如果需要的话
            .select((
                PokemonSpecies::as_select(),
                PokemonColor::as_select(),
                PokemonShape::as_select(),
                // PokemonHabitat::as_select(), // 如果需要的话
            ))
            .limit(pagination.limit())
            .offset(pagination.offset())
            .load::<(PokemonSpecies, PokemonColor, PokemonShape)>(&mut conn)
            .expect("Error loading pokemon species with details");

        // 然后查询所有相关的 egg_groups 关联
        let egg_groups_data = PokemonEggGroup::belonging_to(
            &all_species_with_details
                .iter()
                .map(|(ps, _, _)| ps)
                .collect::<Vec<_>>(),
        )
        .inner_join(egg_groups::table)
        .select((PokemonEggGroup::as_select(), EggGroup::as_select()))
        .load::<(PokemonEggGroup, EggGroup)>(&mut conn)
        .expect("Error loading egg groups");

        // 将 egg_groups 按 species 分组
        let grouped_egg_groups = egg_groups_data.grouped_by(
            &all_species_with_details
                .iter()
                .map(|(ps, _, _)| ps)
                .collect::<Vec<_>>(),
        );

        PaginatedResource {
            data: all_species_with_details
                .into_iter()
                .zip(grouped_egg_groups)
                .map(|((species, color, shape), egg_group_relations)| {
                    let egg_groups = egg_group_relations
                        .into_iter()
                        .map(|(_, egg_group)| egg_group)
                        .collect();

                    PokemonSpecieDetail {
                        specie: species,
                        egg_groups,
                        color,
                        shape,
                        // habitat: habitat,
                    }
                })
                .collect(),
            total_pages,
            total_items,
            page: pagination.page,
            per_page: pagination.per_page,
        }
    }

    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
        use crate::database::schema::pokemon_species::dsl::*;

        let mut conn = self.connection.get().unwrap();

        let (species, color, shape) = pokemon_species
            .find(resource_id)
            .inner_join(pokemon_colors::table)
            .inner_join(pokemon_shapes::table)
            .select((
                PokemonSpecies::as_select(),
                PokemonColor::as_select(),
                PokemonShape::as_select(),
            ))
            .first::<(PokemonSpecies, PokemonColor, PokemonShape)>(&mut conn)
            .ok()?;

        let egg_groups_data = PokemonEggGroup::belonging_to(&species)
            .inner_join(egg_groups::table)
            .select((PokemonEggGroup::as_select(), EggGroup::as_select()))
            .load::<(PokemonEggGroup, EggGroup)>(&mut conn)
            .ok()?;

        let egg_groups = egg_groups_data
            .into_iter()
            .map(|(_, egg_group)| egg_group)
            .collect();

        Some(PokemonSpecieDetail {
            specie: species,
            egg_groups,
            color,
            shape,
            // habitat, // 如果需要的话
        })
    }
}
