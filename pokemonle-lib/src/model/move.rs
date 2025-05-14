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

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::move_damage_classes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["move"])]
pub struct MoveDamageClass {
    pub id: i32,
    pub identifier: String,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::pokemon_moves)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["pokemon", "move"])]
pub struct PokemonMove {
    pub pokemon_id: i32,
    pub version_group_id: i32,
    pub move_id: i32,
    pub pokemon_move_method_id: i32,
    pub level: i32,
    pub order: Option<i32>,
    pub mastery: Option<i32>,
}
