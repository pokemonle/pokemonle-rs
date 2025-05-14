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
    #[diesel(column_name = evolution_trigger_id)]
    pub evolution_trigger: EvolutionTriggerEnum,
    pub trigger_item_id: Option<i32>,
    pub minimum_level: Option<i32>,
    #[diesel(column_name = gender_id)]
    pub gender: Option<Gender>,
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
    diesel::expression::AsExpression,
    diesel::deserialize::FromSqlRow,
)]
#[diesel(sql_type = diesel::sql_types::Integer)]
#[serde(rename_all = "snake_case")]
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

impl<DB> diesel::serialize::ToSql<diesel::sql_types::Integer, DB> for EvolutionTriggerEnum
where
    DB: diesel::backend::Backend,
    i32: diesel::serialize::ToSql<diesel::sql_types::Integer, DB>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, DB>,
    ) -> diesel::serialize::Result {
        match self {
            EvolutionTriggerEnum::LevelUp => 1.to_sql(out),
            EvolutionTriggerEnum::Trade => 2.to_sql(out),
            EvolutionTriggerEnum::UseItem => 3.to_sql(out),
            EvolutionTriggerEnum::Shed => 4.to_sql(out),
            EvolutionTriggerEnum::Spin => 5.to_sql(out),
            EvolutionTriggerEnum::TowerOfDarkness => 6.to_sql(out),
            EvolutionTriggerEnum::TowerOfWaters => 7.to_sql(out),
            EvolutionTriggerEnum::ThreeCriticalHits => 8.to_sql(out),
            EvolutionTriggerEnum::TakeDamage => 9.to_sql(out),
            EvolutionTriggerEnum::Other => 10.to_sql(out),
            EvolutionTriggerEnum::AgileStyleMove => 11.to_sql(out),
            EvolutionTriggerEnum::StrongStyleMove => 12.to_sql(out),
            EvolutionTriggerEnum::RecoilDamage => 13.to_sql(out),
        }
    }
}

impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Integer, DB> for EvolutionTriggerEnum
where
    DB: diesel::backend::Backend,
    i32: diesel::deserialize::FromSql<diesel::sql_types::Integer, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            1 => Ok(EvolutionTriggerEnum::LevelUp),
            2 => Ok(EvolutionTriggerEnum::Trade),
            3 => Ok(EvolutionTriggerEnum::UseItem),
            4 => Ok(EvolutionTriggerEnum::Shed),
            5 => Ok(EvolutionTriggerEnum::Spin),
            6 => Ok(EvolutionTriggerEnum::TowerOfDarkness),
            7 => Ok(EvolutionTriggerEnum::TowerOfWaters),
            8 => Ok(EvolutionTriggerEnum::ThreeCriticalHits),
            9 => Ok(EvolutionTriggerEnum::TakeDamage),
            10 => Ok(EvolutionTriggerEnum::Other),
            11 => Ok(EvolutionTriggerEnum::AgileStyleMove),
            12 => Ok(EvolutionTriggerEnum::StrongStyleMove),
            13 => Ok(EvolutionTriggerEnum::RecoilDamage),
            other => Err(format!("Unexpected variant: {}", other).into()),
        }
    }
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
    diesel::expression::AsExpression,
    diesel::deserialize::FromSqlRow,
)]
#[diesel(sql_type = diesel::sql_types::Integer)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Female = 1,
    Male = 2,
    Genderless = 3,
}

impl<DB> diesel::serialize::ToSql<diesel::sql_types::Integer, DB> for Gender
where
    DB: diesel::backend::Backend,
    i32: diesel::serialize::ToSql<diesel::sql_types::Integer, DB>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, DB>,
    ) -> diesel::serialize::Result {
        match self {
            Gender::Female => 1.to_sql(out),
            Gender::Male => 2.to_sql(out),
            Gender::Genderless => 3.to_sql(out),
        }
    }
}

impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Integer, DB> for Gender
where
    DB: diesel::backend::Backend,
    i32: diesel::deserialize::FromSql<diesel::sql_types::Integer, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            1 => Ok(Gender::Female),
            2 => Ok(Gender::Male),
            3 => Ok(Gender::Genderless),
            other => Err(format!("Unexpected variant: {}", other).into()),
        }
    }
}
