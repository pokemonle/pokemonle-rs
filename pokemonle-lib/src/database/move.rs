use sea_orm::{
    ColumnTrait, EntityTrait, JoinType, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
    RelationTrait,
};

use super::{
    entity::{self, pokemon, pokemon_moves, pokemon_species, pokemon_species_names, prelude::*},
    DatabaseClient,
};
use crate::{
    localized_resource_handler,
    prelude::*,
    types::{prelude::PaginatedResource, WithName},
};

localized_resource_handler!(
    Moves,
    MoveNames,
    entity::move_names::Column::LocalLanguageId,
    entity::move_names::Column::Name
);

impl DatabaseClient {
    pub async fn get_pokemons_by_move_id(
        &self,
        move_id: i32,
        version_group: Option<i32>,
        page: u64,
        limit: u64,
        lang: i32,
    ) -> Result<PaginatedResource<WithName<pokemon::Model>>> {
        let base_query = Pokemon::find()
            .inner_join(PokemonMoves)
            .inner_join(PokemonSpecies)
            .join(
                JoinType::InnerJoin,
                pokemon_species::Relation::PokemonSpeciesNames.def(),
            )
            .filter(pokemon_moves::Column::MoveId.eq(move_id))
            .filter(pokemon_species_names::Column::LocalLanguageId.eq(lang));

        let version_group_query = match version_group {
            Some(vg) => base_query.filter(pokemon_moves::Column::VersionGroupId.eq(vg)),
            None => {
                let max_vg = PokemonMoves::find()
                    .filter(pokemon_moves::Column::MoveId.eq(move_id))
                    .order_by_desc(pokemon_moves::Column::VersionGroupId)
                    .limit(1)
                    .one(&self.conn)
                    .await?;

                match max_vg {
                    Some(vg) => base_query
                        .filter(pokemon_moves::Column::VersionGroupId.eq(vg.version_group_id)),
                    None => base_query,
                }
            }
        };

        let paginator = version_group_query
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
