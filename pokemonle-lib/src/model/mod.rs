use crate::database::schema;
use crate::define_extra_struct;
use aide::OperationIo;
use diesel::prelude::*;

mod berry;
mod encounter;
mod evo;
mod item;
mod location;
mod r#move;
mod pokemon;

pub use berry::*;
pub use encounter::*;
pub use evo::*;
pub use item::*;
pub use location::*;
pub use pokemon::*;
use pokemonle_trait::StructName;
pub use r#move::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::generations)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["generation"])]
pub struct Generation {
    pub id: i32,
    pub identifier: String,
    pub main_region_id: i32,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::languages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["language"])]
pub struct Language {
    pub id: i32,
    pub iso639: String,
    pub iso3166: String,
    pub identifier: String,
    pub official: bool,
    pub order: i32,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::language_names)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["language"])]
pub struct LanguageName {
    pub language_id: i32,
    pub local_language_id: i32,
    pub name: String,
}

#[derive(
    Identifiable,
    Queryable,
    Selectable,
    Serialize,
    Debug,
    Clone,
    JsonSchema,
    StructName,
    OperationIo,
)]
#[diesel(table_name = schema::version_groups)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["version", "version-group"])]
pub struct VersionGroup {
    pub id: i32,
    pub generation_id: i32,
    pub identifier: String,
    pub order: i32,
}

#[derive(
    Identifiable,
    Queryable,
    Selectable,
    Associations,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    JsonSchema,
    StructName,
    OperationIo,
)]
#[diesel(table_name = schema::versions)]
#[diesel(belongs_to(VersionGroup))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["version"])]
pub struct Version {
    pub id: i32,
    pub version_group_id: i32,
    pub identifier: String,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::types)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["type"])]
pub struct Type {
    pub id: i32,
    pub identifier: String,
    pub generation_id: i32,
    pub damage_class_id: Option<i32>,
}

#[derive(
    Queryable, Selectable, Serialize, Deserialize, Debug, Clone, JsonSchema, StructName, OperationIo,
)]
#[diesel(table_name = schema::abilities)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["ability"])]
pub struct Ability {
    pub id: i32,
    pub identifier: String,
    pub generation_id: i32,
    pub is_main_series: Option<bool>,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::growth_rates)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
pub struct GrowthRate {
    pub id: i32,
    pub identifier: String,
    pub formula: String,
}

#[derive(
    Identifiable,
    Queryable,
    Selectable,
    Serialize,
    Debug,
    Clone,
    JsonSchema,
    StructName,
    OperationIo,
)]
#[diesel(table_name = schema::egg_groups)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["egg_group"])]
pub struct EggGroup {
    pub id: i32,
    pub identifier: String,
}

#[derive(
    Identifiable,
    Queryable,
    Selectable,
    Serialize,
    Debug,
    Clone,
    JsonSchema,
    StructName,
    OperationIo,
)]
#[diesel(table_name = schema::contest_types)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["contest"])]
pub struct ContestType {
    pub id: i32,
    pub identifier: String,
}

#[derive(
    Identifiable,
    Queryable,
    Selectable,
    Serialize,
    Debug,
    Clone,
    JsonSchema,
    StructName,
    OperationIo,
)]
#[diesel(table_name = schema::contest_effects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["contest"])]
pub struct ContestEffect {
    pub id: i32,
    pub appeal: i32,
    pub jam: i32,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::pokedexes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["pokedex"])]
pub struct Pokedex {
    pub id: i32,
    pub identifier: String,
    pub is_main_series: bool,
    pub region_id: Option<i32>,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::pokedex_version_groups)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["pokedex"])]
pub struct PokedexVersionGroup {
    pub pokedex_id: i32,
    pub version_group_id: i32,
}

define_extra_struct!(Languaged { name: String });

impl<T> Languaged<T>
where
    T: StructName,
{
    pub fn new(item: T, name: String) -> Self {
        Self { name, item }
    }

    pub fn new_from_tuple(tuple: (T, String)) -> Self {
        Self::new(tuple.0, tuple.1)
    }
}

define_extra_struct!(WithSlot {
    slot: i32,
    is_hidden: bool
});

define_extra_struct!(WithVersions {
    versions: Vec<Languaged<Version>>
});

#[derive(Debug, Serialize, Deserialize, JsonSchema, OperationIo)]
#[serde(rename_all = "snake_case")]
pub enum DescriptionVersion {
    Version(i32),
    VersionGroup(i32),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, StructName, OperationIo)]
pub struct ResourceDescription {
    pub description: String,
    #[serde(flatten)]
    pub version: DescriptionVersion,
    pub language: i32,
}
