use crate::database::schema;
use aide::OperationIo;
use diesel::prelude::*;

mod evo;
mod item;
mod pokemon;
pub use evo::*;
pub use item::*;
pub use pokemon::*;
use pokemonle_trait::StructName;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::generations)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
pub struct Generation {
    pub id: i32,
    pub main_region_id: i32,
    pub identifier: String,
}

#[derive(Queryable, Selectable, Serialize, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::languages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
pub struct Language {
    pub id: i32,
    pub iso639: String,
    pub iso3166: String,
    pub identifier: String,
    pub official: bool,
    pub order: i32,
}

#[derive(Queryable, Selectable, Serialize, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::version_groups)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
pub struct VersionGroup {
    pub id: i32,
    pub generation_id: i32,
    pub identifier: String,
    pub order: i32,
}

#[derive(Queryable, Selectable, Serialize, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::versions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
pub struct Version {
    pub id: i32,
    pub version_group_id: i32,
    pub identifier: String,
}

#[derive(Queryable, Selectable, Serialize, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::move_damage_classes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
pub struct MoveDamageClass {
    pub id: i32,
    pub identifier: String,
}

#[derive(Queryable, Selectable, Serialize, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::types)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
pub struct Type {
    pub id: i32,
    pub identifier: String,
    pub generation_id: i32,
    pub damage_class_id: Option<i32>,
}

#[derive(Queryable, Selectable, Serialize, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::abilities)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
pub struct Ability {
    pub id: i32,
    pub identifier: String,
    pub generation_id: i32,
    pub is_main_series: Option<bool>,
}

#[derive(Queryable, Selectable, Serialize, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::growth_rates)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
pub struct GrowthRate {
    pub id: i32,
    pub identifier: String,
    pub formula: String,
}
