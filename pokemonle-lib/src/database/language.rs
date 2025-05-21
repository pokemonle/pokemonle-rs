use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

use super::{
    entity::{self, language_names, prelude::*},
    DatabaseClient,
};
use crate::{localized_resource_handler, prelude::*, types::prelude::PaginatedResource};

localized_resource_handler!(
    Languages,
    LanguageNames,
    entity::language_names::Column::LocalLanguageId,
    entity::language_names::Column::Name
);

impl DatabaseClient {
    pub async fn get_local_languages(&self) -> Result<PaginatedResource<language_names::Model>> {
        use entity::language_names::Column;
        use sea_orm::prelude::*;
        let languages = LanguageNames::find()
            .filter(
                Column::LocalLanguageId
                    .into_expr()
                    .eq(Column::LanguageId.into_expr()),
            )
            .all(&self.conn)
            .await
            .map_err(Error::ConnectionError)?;

        Ok(PaginatedResource::new_from_vec(languages))
    }

    pub async fn get_ability_all_flavor_text(
        &self,
        id: i32,
        lang: i32,
    ) -> Result<PaginatedResource<entity::ability_flavor_text::Model>> {
        AbilityFlavorText::find()
            .filter(entity::ability_flavor_text::Column::AbilityId.eq(id))
            .filter(entity::ability_flavor_text::Column::LanguageId.eq(lang))
            .all(&self.conn)
            .await
            .map_err(Error::ConnectionError)
            .map(PaginatedResource::new_from_vec)
    }

    pub async fn get_ability_flavor_text(
        &self,
        id: i32,
        version_group: Option<i32>,
        lang: i32,
    ) -> Result<entity::ability_flavor_text::Model> {
        let select = match version_group {
            Some(vg) => AbilityFlavorText::find_by_id((id, vg, lang)),
            None => AbilityFlavorText::find()
                .filter(entity::ability_flavor_text::Column::AbilityId.eq(id))
                .filter(entity::ability_flavor_text::Column::LanguageId.eq(lang))
                .order_by_desc(entity::ability_flavor_text::Column::VersionGroupId),
        };

        select
            .one(&self.conn)
            .await
            .map_err(Error::ConnectionError)?
            .ok_or(Error::ResourceNotFound(format!(
                "Ability flavor text not found for id: {} and lang: {}",
                id, lang
            )))
    }

    pub async fn get_item_flavor_text(
        &self,
        id: i32,
        version_group: i32,
        lang: i32,
    ) -> Result<entity::item_flavor_text::Model> {
        ItemFlavorText::find_by_id((id, version_group, lang))
            .one(&self.conn)
            .await
            .map_err(Error::ConnectionError)?
            .ok_or(Error::ResourceNotFound(format!(
                "Item flavor text not found for id: {} and lang: {}",
                id, lang
            )))
    }

    pub async fn get_move_flavor_text(
        &self,
        id: i32,
        version_group: Option<i32>,
        lang: i32,
    ) -> Result<entity::move_flavor_text::Model> {
        let select = match version_group {
            Some(vg) => MoveFlavorText::find_by_id((id, vg, lang)),
            None => MoveFlavorText::find()
                .filter(entity::move_flavor_text::Column::MoveId.eq(id))
                .filter(entity::move_flavor_text::Column::LanguageId.eq(lang))
                .order_by_desc(entity::move_flavor_text::Column::VersionGroupId),
        };

        select
            .one(&self.conn)
            .await
            .map_err(Error::ConnectionError)?
            .ok_or(Error::ResourceNotFound(format!(
                "Move flavor text not found for id: {} and lang: {}",
                id, lang
            )))
    }

    pub async fn get_pokemon_species_flavor_text(
        &self,
        id: i32,
        version_group: i32,
        lang: i32,
    ) -> Result<entity::pokemon_species_flavor_text::Model> {
        PokemonSpeciesFlavorText::find_by_id((id, version_group, lang))
            .one(&self.conn)
            .await
            .map_err(Error::ConnectionError)?
            .ok_or(Error::ResourceNotFound(format!(
                "Pokemon species flavor text not found for id: {} and lang: {}",
                id, lang
            )))
    }
}
