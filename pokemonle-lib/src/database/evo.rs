use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use super::{
    entity::{pokemon_species, pokemon_species_names, prelude::*},
    DatabaseClient,
};
use crate::{
    prelude::*,
    types::{
        prelude::{PaginateQuery, PaginatedResource},
        WithName,
    },
};

impl DatabaseClient {
    pub async fn get_pokemon_species_by_evolution_chain_id(
        &self,
        evolution_chain_id: i32,
        lang: i32,
    ) -> Result<PaginatedResource<WithName<pokemon_species::Model>>> {
        let result = PokemonSpecies::find()
            .find_also_related(PokemonSpeciesNames)
            .filter(pokemon_species::Column::EvolutionChainId.eq(evolution_chain_id))
            .filter(pokemon_species_names::Column::LocalLanguageId.eq(lang))
            .all(&self.conn)
            .await?;

        let data: Vec<_> = result
            .into_iter()
            .map(|(p, n)| WithName {
                name: if let Some(n) = n {
                    n.name
                } else {
                    p.identifier.clone()
                },
                item: p,
            })
            .collect();

        let total_items = data.len() as u64;

        Ok(PaginatedResource {
            data,
            total_pages: 1,
            total_items,
            page: PaginateQuery::default().page,
            per_page: PaginateQuery::default().per_page,
        })
    }
}
