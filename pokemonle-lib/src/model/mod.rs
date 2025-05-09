use crate::database::schema;
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
    pub main_region_id: i32,
    pub identifier: String,
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
#[diesel(table_name = schema::version_groups)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["version", "version-group"])]
pub struct VersionGroup {
    pub id: i32,
    pub generation_id: i32,
    pub identifier: String,
    pub order: i32,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::versions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["version"])]
pub struct Version {
    pub id: i32,
    pub version_group_id: i32,
    pub identifier: String,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::move_damage_classes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["move"])]
pub struct MoveDamageClass {
    pub id: i32,
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

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
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

#[derive(OperationIo, Serialize, Deserialize, JsonSchema, Clone)]
pub struct Languaged<T: StructName + Serialize> {
    #[serde(flatten)]
    pub item: T,
    pub name: String,
    // pub description: String,
}

impl<T: StructName + Serialize> StructName for Languaged<T> {
    fn struct_name() -> &'static str {
        T::struct_name()
    }
}
