use sea_orm::{
    ColumnTrait, EntityTrait, JoinType, PaginatorTrait, QueryFilter, QuerySelect, RelationTrait,
};

use super::{
    entity::{
        self, pokemon, pokemon_abilities, pokemon_species, pokemon_species_names, prelude::*,
    },
    DatabaseClient,
};
use crate::{
    localized_resource_handler,
    prelude::*,
    types::{prelude::PaginatedResource, WithName},
};

localized_resource_handler!(
    Abilities,
    AbilityNames,
    entity::ability_names::Column::LocalLanguageId,
    entity::ability_names::Column::Name
);

impl DatabaseClient {
    pub async fn get_pokemons_by_ability_id(
        &self,
        ability_id: i32,
        page: u64,
        limit: u64,
        lang: i32,
    ) -> Result<PaginatedResource<WithName<pokemon::Model>>> {
        // find pokemons from pokemon_abilties table where ability_id = ability_id

        let paginator = Pokemon::find()
            .inner_join(PokemonAbilities)
            .inner_join(PokemonSpecies)
            .join(
                JoinType::InnerJoin,
                pokemon_species::Relation::PokemonSpeciesNames.def(),
            )
            .filter(pokemon_abilities::Column::AbilityId.eq(ability_id))
            .filter(pokemon_species_names::Column::LocalLanguageId.eq(lang))
            .select_also(PokemonSpeciesNames)
            .paginate(&self.conn, limit);

        let data = paginator.fetch_page(page - 1).await?;

        let total = paginator.num_items_and_pages().await?;

        Ok(PaginatedResource {
            data: data
                .into_iter()
                .map(|(p, n)| WithName {
                    name: if let Some(n) = n {
                        n.name
                    } else {
                        p.identifier.clone()
                    },
                    item: p,
                })
                .collect(),
            page,
            per_page: limit,
            total_pages: total.number_of_pages,
            total_items: total.number_of_items,
        })
    }
}
