mod berry;
mod contest;
mod encounter;
mod evolution;
mod item;
mod language;
mod location;
mod pokemon;

use crate::config::Config;
use crate::model::ResourceDescription;
use crate::{impl_handlers, prelude::*};
use diesel::migration::{MigrationVersion, Result as MigrationResult};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{Connection, MultiConnection, PgConnection, QueryResult, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::sync::{Mutex, Once};
use tracing::debug;

use super::pagination::PaginatedResource;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

static VFS: Mutex<(i32, Once)> = Mutex::new((0, Once::new()));

#[derive(MultiConnection)]
pub enum DatabaseConnection {
    Pg(PgConnection),
    Sqlite(SqliteConnection),
}

pub struct DatabaseClient {
    pub connection: DatabaseConnection,
}

impl DatabaseClient {
    pub fn new() -> Result<Self> {
        let config = Config::new()?;
        let connection = {
            let url = &config.database_url;
            if url.starts_with("postgres://") {
                DatabaseConnection::Pg(PgConnection::establish(url)?)
            } else {
                DatabaseConnection::Sqlite(SqliteConnection::establish(url)?)
            }
        };

        Ok(DatabaseClient { connection })
    }
}

pub trait DatabaseHandler {
    type Resource;
    fn get_all_resources(
        &self,
        pagination: super::pagination::Paginated,
    ) -> PaginatedResource<Self::Resource>;
    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource>;
}

pub trait DatabaseHandlerWithLocale {
    type Resource;
    fn get_all_resources_with_locale(
        &self,
        pagination: super::pagination::Paginated,
        locale_id: i32,
        query: Option<String>,
    ) -> PaginatedResource<(Self::Resource, String)>;
    fn get_resource_by_id_with_locale(
        &self,
        resource_id: i32,
        locale_id: i32,
    ) -> Option<(Self::Resource, String)>;
}

pub trait DatabaseHandlerWithFlavorText {
    fn get_all_resources_with_flavor_text(
        &self,
        resource_id: i32,
        pagination: super::pagination::Paginated,
        locale_id: i32,
    ) -> PaginatedResource<ResourceDescription>;

    fn get_latest_flavor_text(
        &self,
        resource_id: i32,
        locale_id: i32,
    ) -> Option<ResourceDescription>;
}

#[derive(Clone)]
pub struct DatabaseClientPooled {
    connection: Pool<ConnectionManager<DatabaseConnection>>,
}

impl DatabaseClientPooled {
    pub fn new() -> Result<Self> {
        let config = Config::new()?;

        let pool = if config.database_url.starts_with("postgres://") {
            Pool::builder().build(ConnectionManager::new(config.database_url))?
        } else {
            let (vfs, once) = &*VFS.lock().unwrap();
            let url = match vfs {
                0 => &config.database_url,
                1 => &format!("file:{}?vfs=opfs-sahpool", config.database_url),
                2 => &format!("file:{}?vfs=relaxed-idb", config.database_url),
                _ => unreachable!(),
            };

            let mut conn = SqliteConnection::establish(url)
                .unwrap_or_else(|_| panic!("{}", format!("Error connecting to {}", url)));
            once.call_once(|| {
                // Run migrations
                let r = conn
                    .pending_migrations(MIGRATIONS)
                    .expect("Error loading migrations")
                    .iter()
                    .map(|m| {
                        debug!("Running migration: {}", m.name());
                        conn.run_migration(m)
                    })
                    .collect::<MigrationResult<Vec<MigrationVersion>>>();

                r.unwrap();
            });

            Pool::builder().build(ConnectionManager::new(config.database_url))?
        };

        Ok(DatabaseClientPooled { connection: pool })
    }
}

impl_handlers! {
    ability: ability::AbilityHandler,
    berry: berry::BerryHandler,
    berry_firmness: berry::BerryFirmnessHandler,
    contest_effect: contest::ContestEffectHandler,
    contest_type: contest::ContestTypeHandler,
    encounter: encounter::EncounterHandler,
    encounter_condition: encounter::EncounterConditionHandler,
    encounter_condition_value: encounter::EncounterConditionValueHandler,
    encounter_slot: encounter::EncounterSlotHandler,
    encounter_method: encounter::EncounterMethodHandler,
    evolution_chain: evolution::EvolutionChainHandler,
    evolution_trigger: evolution::EvolutionTriggerHandler,
    generation: generation::GenerationHandler,
    location: location::LocationHandler,
    location_area: location::LocationAreaHandler,
    r#move: r#move::MoveHandler,
    region: location::RegionHandler,
    item: item::ItemHandler,
    item_category: item::ItemCategoryHandler,
    item_pocket: item::ItemPocketHandler,
    language: language::LanguageHandler,
    pokedex: pokedex::PokedexHandler,
    pokemon: pokemon::PokemonHandler,
    pokemon_color: pokemon::PokemonColorHandler,
    pokemon_habitat: pokemon::PokemonHabitatHandler,
    pokemon_shape: pokemon::PokemonShapeHandler,
    pokemon_specie: pokemon::PokemonSpecieHandler,
    r#type: r#type::TypeHandler,
    version: version::VersionHandler,
    version_group: version_group::VersionGroupHandler,
}

mod ability {
    use crate::model::Ability;
    use crate::{
        impl_database_flavor_text_handler, impl_database_handler, impl_database_locale_handler,
    };

    use crate::database::schema::{abilities, ability_flavor_text, ability_names};
    impl_database_handler!(
        AbilityHandler,
        Ability,
        abilities::dsl::abilities,
        abilities::dsl::id
    );

    impl_database_locale_handler!(
        AbilityHandler,
        Ability,
        abilities::dsl::abilities,
        abilities::dsl::id,
        ability_names::dsl::ability_names,
        ability_names::dsl::ability_id,
        ability_names::dsl::name,
        ability_names::dsl::local_language_id
    );

    impl_database_flavor_text_handler!(
        AbilityHandler,
        ability_flavor_text::dsl::ability_flavor_text,
        ability_flavor_text::dsl::ability_id,
        ability_flavor_text::dsl::flavor_text,
        ability_flavor_text::dsl::language_id,
        ability_flavor_text::dsl::version_group_id
    );
}

mod generation {
    use crate::database::schema::{generation_names, generations};
    use crate::model::Generation;
    use crate::{impl_database_handler, impl_database_locale_handler};

    impl_database_handler!(
        GenerationHandler,
        Generation,
        generations::dsl::generations,
        generations::dsl::id
    );

    impl_database_locale_handler!(
        GenerationHandler,
        Generation,
        generations::dsl::generations,
        generations::dsl::id,
        generation_names::dsl::generation_names,
        generation_names::dsl::generation_id,
        generation_names::dsl::name,
        generation_names::dsl::local_language_id
    );
}

mod pokedex {
    use crate::database::schema::pokedexes;
    use crate::impl_database_handler;
    use crate::model::Pokedex;

    impl_database_handler!(
        PokedexHandler,
        Pokedex,
        pokedexes::dsl::pokedexes,
        pokedexes::dsl::id
    );
}

mod r#type {
    use crate::database::schema::{type_names, types};
    use crate::model::Type;
    use crate::{impl_database_handler, impl_database_locale_handler};

    impl_database_handler!(TypeHandler, Type, types::dsl::types, types::dsl::id);

    impl_database_locale_handler!(
        TypeHandler,
        Type,
        types::dsl::types,
        types::dsl::id,
        type_names::dsl::type_names,
        type_names::dsl::type_id,
        type_names::dsl::name,
        type_names::dsl::local_language_id
    );
}

mod version {
    use crate::database::schema::{version_names, versions};
    use crate::model::Version;
    use crate::{impl_database_handler, impl_database_locale_handler};

    impl_database_handler!(
        VersionHandler,
        Version,
        versions::dsl::versions,
        versions::dsl::id
    );

    impl_database_locale_handler!(
        VersionHandler,
        Version,
        versions::dsl::versions,
        versions::dsl::id,
        version_names::dsl::version_names,
        version_names::dsl::version_id,
        version_names::dsl::name,
        version_names::dsl::local_language_id
    );
}

mod version_group {
    use crate::database::schema::version_groups;
    use crate::impl_database_handler;
    use crate::model::VersionGroup;

    impl_database_handler!(
        VersionGroupHandler,
        VersionGroup,
        version_groups::dsl::version_groups,
        version_groups::dsl::id
    );
}

mod r#move {
    use crate::database::schema::{move_flavor_text, move_names, moves};
    use crate::model::Move;
    use crate::{
        impl_database_flavor_text_handler, impl_database_handler, impl_database_locale_handler,
    };

    impl_database_handler!(MoveHandler, Move, moves::dsl::moves, moves::dsl::id);

    impl_database_locale_handler!(
        MoveHandler,
        Move,
        moves::dsl::moves,
        moves::dsl::id,
        move_names::dsl::move_names,
        move_names::dsl::move_id,
        move_names::dsl::name,
        move_names::dsl::local_language_id
    );

    impl_database_flavor_text_handler!(
        MoveHandler,
        move_flavor_text::dsl::move_flavor_text,
        move_flavor_text::dsl::move_id,
        move_flavor_text::dsl::flavor_text,
        move_flavor_text::dsl::language_id,
        move_flavor_text::dsl::version_group_id
    );
}
