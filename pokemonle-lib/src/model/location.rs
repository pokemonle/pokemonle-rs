use crate::database::schema;
use aide::OperationIo;
use diesel::prelude::*;

use pokemonle_trait::StructName;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::locations)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["location"])]
pub struct Location {
    pub id: i32,
    pub region_id: Option<i32>,
    pub identifier: String,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::location_areas)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["location"])]
pub struct LocationArea {
    pub id: i32,
    pub location_id: i32,
    pub game_index: i32,
    pub identifier: String,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::regions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["location"])]
pub struct Region {
    pub id: i32,
    pub identifier: String,
}
