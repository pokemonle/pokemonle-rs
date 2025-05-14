use crate::database::schema;
use aide::OperationIo;
use diesel::prelude::*;
use pokemonle_trait::StructName;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
    StructName,
    OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub enum PokemonMoveMethod {
    LevelUp = 1,
    Egg = 2,
    Tutor = 3,
    Machine = 4,
    StadiumSurfingPikachu = 5,
    LightBallEgg = 6,
    ColossenumPurification = 7,
    XdShadow = 8,
    XdPurification = 9,
    FormChange = 10,
    ZygardeCube = 11,
    Unknown,
}
