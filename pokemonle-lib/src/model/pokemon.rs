use crate::database::schema;
use aide::OperationIo;
use diesel::prelude::*;

use pokemonle_trait::StructName;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::pokemon_colors)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
pub struct PokemonColor {
    pub id: i32,
    pub identifier: String,
}

#[derive(Queryable, Selectable, Serialize, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::pokemon_shapes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
pub struct PokemonShape {
    pub id: i32,
    pub identifier: String,
}

#[derive(Queryable, Selectable, Serialize, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::pokemon_habitats)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
pub struct PokemonHabitat {
    pub id: i32,
    pub identifier: String,
}

#[derive(Queryable, Selectable, Serialize, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::pokemon)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
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

#[derive(Queryable, Selectable, Serialize, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::pokemon_species)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
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
