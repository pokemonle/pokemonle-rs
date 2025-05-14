use crate::database::schema;
use aide::OperationIo;
use diesel::prelude::*;

use pokemonle_trait::StructName;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::pokemon_colors)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["pokemon", "color"])]
pub struct PokemonColor {
    pub id: i32,
    pub identifier: String,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::pokemon_shapes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["pokemon", "shape"])]
pub struct PokemonShape {
    pub id: i32,
    pub identifier: String,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::pokemon_habitats)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["pokemon", "habitat"])]
pub struct PokemonHabitat {
    pub id: i32,
    pub identifier: String,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::pokemon)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["pokemon"])]
pub struct Pokemon {
    pub id: i32,
    pub identifier: String,
    pub species_id: i32,
    pub height: i32,
    pub weight: i32,
    pub base_experience: i32,
    pub order: Option<i32>,
    pub is_default: bool,
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
#[diesel(belongs_to(PokemonColor, foreign_key = color_id))]
#[diesel(table_name = schema::pokemon_species)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["pokemon"])]
pub struct PokemonSpecies {
    pub id: i32,
    pub identifier: String,
    pub generation_id: i32,
    pub evolution_chain_id: Option<i32>,
    pub color_id: i32,
    pub shape_id: i32,
    pub habitat_id: Option<i32>,
    pub gender_rate: Option<i32>,
    pub capture_rate: Option<i32>,
    pub base_happiness: Option<i32>,
    pub is_baby: bool,
    pub hatch_counter: i32,
    pub has_gender_differences: bool,
    pub growth_rate_id: i32,
    pub forms_switchable: bool,
    pub is_legendary: bool,
    pub is_mythical: bool,
    pub order: i32,
    pub conquest_order: Option<i32>,
}

#[derive(
    Queryable,
    Selectable,
    Associations,
    Serialize,
    Debug,
    Clone,
    JsonSchema,
    StructName,
    OperationIo,
)]
#[diesel(table_name = schema::pokemon_abilities)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[diesel(belongs_to(Pokemon))]
#[diesel(belongs_to(super::Ability))]
#[pokemonle(tags = ["pokemon", "ability"])]
pub struct PokemonAbility {
    pub pokemon_id: i32,
    pub ability_id: i32,
    pub is_hidden: bool,
    pub slot: i32,
}

#[derive(
    Identifiable,
    Queryable,
    Selectable,
    Associations,
    Serialize,
    Debug,
    Clone,
    JsonSchema,
    StructName,
    OperationIo,
)]
#[diesel(table_name = schema::pokemon_egg_groups)]
#[diesel(belongs_to(PokemonSpecies,foreign_key = species_id))]
#[diesel(belongs_to(super::EggGroup))]
#[diesel(primary_key(species_id, egg_group_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["pokemon", "egg_group"])]
pub struct PokemonEggGroup {
    pub species_id: i32,
    pub egg_group_id: i32,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::pokemon_stats)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["pokemon", "stat"])]
pub struct PokemonStat {
    pub pokemon_id: i32,
    pub stat_id: i32,
    pub base_stat: i32,
    pub effort: i32,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::pokemon_types)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["pokemon", "type"])]
pub struct PokemonType {
    pub pokemon_id: i32,
    pub type_id: i32,
    pub slot: i32,
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

#[derive(Queryable, OperationIo, StructName, Serialize, JsonSchema, Clone)]
#[pokemonle(tags = ["pokemon"])]
pub struct PokemonDetail {
    #[serde(flatten)]
    pub pokemon: Pokemon,
    pub abilities: Vec<PokemonAbility>,
}
