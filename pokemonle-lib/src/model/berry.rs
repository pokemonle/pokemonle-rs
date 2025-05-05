use crate::database::schema;
use aide::OperationIo;
use diesel::prelude::*;
use pokemonle_trait::StructName;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::berry_firmness)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["berry"])]
pub struct BerryFirmness {
    pub id: i32,
    pub identifier: String,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::berries)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["berry"])]
pub struct Berry {
    pub id: i32,
    pub item_id: i32,
    pub firmness_id: i32,
    pub natural_gift_power: i32,
    pub natural_gift_type_id: i32,
    pub size: i32,
    pub max_harvest: i32,
    pub growth_time: i32,
    pub soil_dryness: i32,
    pub smoothness: i32,
}
