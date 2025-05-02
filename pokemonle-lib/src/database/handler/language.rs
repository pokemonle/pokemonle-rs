use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use super::DatabaseConnection;
use crate::database::schema::languages::dsl::*;
use crate::model::Language;

pub struct LanguageHandler {
    pub connection: Pool<ConnectionManager<DatabaseConnection>>,
}

impl LanguageHandler {
    pub fn new(connection: Pool<ConnectionManager<DatabaseConnection>>) -> Self {
        LanguageHandler { connection }
    }

    pub fn get_all_languages(&self) -> Vec<Language> {
        languages
            .select(Language::as_select())
            .load(&mut self.connection.get().unwrap())
            .expect("Error loading languages")
    }

    pub fn get_language_by_id(&self, language_id: i32) -> Option<Language> {
        languages
            .filter(id.eq(language_id))
            .select(Language::as_select())
            .first::<Language>(&mut self.connection.get().unwrap())
            .ok()
    }
}
