use super::{
    entity::{self, language_names, prelude::*},
    DatabaseClient,
};
use crate::{
    localized_resource_handler,
    prelude::*,
    types::{prelude::PaginatedResource, WithName},
};

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
}
