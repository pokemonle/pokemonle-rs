use crate::database::schema;
use aide::OperationIo;
use diesel::prelude::*;

use pokemonle_trait::StructName;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::evolution_chains)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["evolution"])]
pub struct EvolutionChain {
    pub id: i32,
    pub baby_trigger_item_id: Option<i32>,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::evolution_triggers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["evolution"])]
pub struct EvolutionTrigger {
    pub id: i32,
    pub identifier: String,
}
