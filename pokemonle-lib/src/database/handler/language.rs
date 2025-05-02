use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use super::{DatabaseConnection, DatabaseHandler};
use crate::database::schema::languages::dsl::*;
use crate::model::Language;

pub struct LanguageHandler {
    pub connection: Pool<ConnectionManager<DatabaseConnection>>,
}

impl LanguageHandler {
    pub fn new(connection: Pool<ConnectionManager<DatabaseConnection>>) -> Self {
        LanguageHandler { connection }
    }
}

impl DatabaseHandler for LanguageHandler {
    type Resource = Language;

    fn get_all_resources(&self) -> Vec<Self::Resource> {
        languages
            .select(Language::as_select())
            .load(&mut self.connection.get().unwrap())
            .expect("Error loading languages")
    }

    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
        languages
            .filter(id.eq(resource_id))
            .select(Language::as_select())
            .first::<Language>(&mut self.connection.get().unwrap())
            .ok()
    }
}
