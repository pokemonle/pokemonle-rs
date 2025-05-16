use crate::{
    prelude::*,
    types::{response::PaginatedResource, WithName},
};
use async_trait::async_trait;
use sea_orm::{
    ColumnTrait, EntityTrait, FromQueryResult, Paginator, PaginatorTrait, PrimaryKeyTrait,
    QueryFilter,
};

use super::{
    entry::{
        pokemon_species::Entity as PokemonSpecies,
        pokemon_species_names::{Column::LocalLanguageId, Entity as PokemonSpeciesName},
    },
    r#trait::LocalizedResourceHandler,
    DatabaseClient,
};

#[async_trait]
impl LocalizedResourceHandler<PokemonSpecies> for DatabaseClient {
    async fn list_with_pagination(
        &self,
        page: u64,
        limit: u64,
        lang: i32,
    ) -> Result<PaginatedResource<WithName<<PokemonSpecies as EntityTrait>::Model>>> {
        let paginator = PokemonSpecies::find()
            .find_also_related(PokemonSpeciesName)
            .filter(LocalLanguageId.eq(lang))
            .paginate(&self.conn, limit);

        let data = paginator
            .fetch_page(page)
            .await?
            .into_iter()
            .map(|(pokemon, pokemon_species_name)| {
                let name = match pokemon_species_name {
                    Some(psn) => psn.name,
                    None => pokemon.identifier.clone(),
                };

                WithName::new(pokemon, name)
            })
            .collect();

        Ok(PaginatedResource {
            data,
            page,
            per_page: limit,
            total_pages: paginator.num_pages().await?,
            total_items: paginator.num_items().await?,
        })
    }

    async fn get_by_id(
        &self,
        id: i32,
        lang: i32,
    ) -> Result<WithName<<PokemonSpecies as EntityTrait>::Model>> {
        let (pokemon, pokemon_species_name) = PokemonSpecies::find_by_id(id)
            .find_also_related(PokemonSpeciesName)
            .filter(LocalLanguageId.eq(lang))
            .one(&self.conn)
            .await
            .map_err(Error::ConnectionError)?
            .ok_or(Error::ResourceNotFound(format!(
                "resource {} not found",
                id
            )))?;

        let name = match pokemon_species_name {
            Some(psn) => psn.name,
            None => pokemon.identifier.clone(),
        };

        Ok(WithName::new(pokemon, name))
    }
}
