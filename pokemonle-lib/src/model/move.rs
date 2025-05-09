use crate::database::schema;
use aide::OperationIo;
use diesel::prelude::*;
use pokemonle_trait::StructName;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::moves)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["move"])]
pub struct Move {
    pub id: i32,
    pub identifier: String,
    pub generation_id: i32,
    pub type_id: Option<i32>,
    pub power: Option<i32>,
    pub pp: Option<i32>,
    pub accuracy: Option<i32>,
    pub priority: i32,
    pub target_id: i32,
    pub damage_class_id: i32,
    pub effect_id: Option<i32>,
    pub effect_chance: Option<i32>,
    pub contest_type_id: Option<i32>,
    pub contest_effect_id: Option<i32>,
}
