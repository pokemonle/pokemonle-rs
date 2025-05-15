use crate::database::schema::{language_names, languages};
use crate::model::{Language, LanguageName};
use crate::prelude::*;
use crate::types::response::PaginatedResource;
use crate::{impl_database_handler, impl_database_locale_handler};

impl_database_handler!(
    LanguageHandler,
    Language,
    languages::dsl::languages,
    languages::dsl::id
);

impl_database_locale_handler!(
    LanguageHandler,
    Language,
    languages::dsl::languages,
    languages::dsl::id,
    language_names::dsl::language_names,
    language_names::dsl::language_id,
    language_names::dsl::name,
    language_names::dsl::local_language_id
);

impl LanguageHandler {
    pub fn get_local_lanuages(&self) -> Result<PaginatedResource<LanguageName>> {
        use crate::database::schema::language_names::dsl::*;
        use diesel::prelude::*;
        language_names
            .filter(local_language_id.eq(language_id))
            .load::<LanguageName>(&mut self.connection.get().map_err(Error::R2D2PoolError)?)
            .map_err(Error::DieselError)
            .map(PaginatedResource::new_from_vec)
    }
}
