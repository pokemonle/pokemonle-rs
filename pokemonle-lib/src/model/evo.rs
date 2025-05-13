use crate::database::schema;
use aide::OperationIo;
use diesel::prelude::*;

use pokemonle_trait::StructName;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::pokemon_evolution)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["pokemon", "evolution"])]
pub struct PokemonEvolution {
    pub id: i32,
    pub evolved_species_id: i32,
    pub evolution_trigger_id: i32,
    pub trigger_item_id: Option<i32>,
    pub minimum_level: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EvolutionTriggerEnum {
    LevelUp = 1,
    Trade = 2,
    UseItem = 3,
    Shed = 4,
    Spin = 5,
    TowerOfDarkness = 6,
    TowerOfWaters = 7,
    ThreeCriticalHits = 8,
    TakeDamage = 9,
    Other = 10,
    AgileStyleMove = 11,
    StrongStyleMove = 12,
    RecoilDamage = 13,
}

impl TryFrom<i32> for EvolutionTriggerEnum {
    type Error = String;

    fn try_from(id: i32) -> Result<Self, Self::Error> {
        match id {
            1 => Ok(Self::LevelUp),
            2 => Ok(Self::Trade),
            3 => Ok(Self::UseItem),
            4 => Ok(Self::Shed),
            5 => Ok(Self::Spin),
            6 => Ok(Self::TowerOfDarkness),
            7 => Ok(Self::TowerOfWaters),
            8 => Ok(Self::ThreeCriticalHits),
            9 => Ok(Self::TakeDamage),
            10 => Ok(Self::Other),
            11 => Ok(Self::AgileStyleMove),
            12 => Ok(Self::StrongStyleMove),
            13 => Ok(Self::RecoilDamage),
            _ => Err(format!("Invalid evolution trigger id: {}", id)),
        }
    }
}

impl EvolutionTriggerEnum {
    pub fn identifier(&self) -> &str {
        match self {
            Self::LevelUp => "level-up",
            Self::Trade => "trade",
            Self::UseItem => "use-item",
            Self::Shed => "shed",
            Self::Spin => "spin",
            Self::TowerOfDarkness => "tower-of-darkness",
            Self::TowerOfWaters => "tower-of-waters",
            Self::ThreeCriticalHits => "three-critical-hits",
            Self::TakeDamage => "take-damage",
            Self::Other => "other",
            Self::AgileStyleMove => "agile-style-move",
            Self::StrongStyleMove => "strong-style-move",
            Self::RecoilDamage => "recoil-damage",
        }
    }
}
