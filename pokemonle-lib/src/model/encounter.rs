use crate::database::schema;
use aide::OperationIo;
use diesel::prelude::*;
use pokemonle_trait::StructName;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::encounter_methods)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["encounter"])]
pub struct EncounterMethod {
    pub id: i32,
    pub identifier: String,
    pub order: i32,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::encounter_slots)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["encounter"])]
pub struct EncounterSlot {
    pub id: i32,
    pub version_group_id: i32,
    pub encounter_method_id: i32,
    pub slot: Option<i32>,
    pub rarity: i32,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::encounters)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["encounter"])]
pub struct Encounter {
    pub id: i32,
    pub version_id: i32,
    pub location_area_id: i32,
    pub encounter_slot_id: i32,
    pub pokemon_id: i32,
    pub min_level: i32,
    pub max_level: i32,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::encounter_conditions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["encounter"])]
pub struct EncounterCondition {
    pub id: i32,
    pub identifier: String,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::encounter_condition_values)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["encounter"])]
pub struct EncounterConditionValue {
    pub id: i32,
    pub encounter_condition_id: i32,
    pub identifier: String,
    pub is_default: bool,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::encounter_condition_value_map)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["encounter"])]
pub struct EncounterConditionValueMap {
    pub encounter_id: i32,
    pub encounter_condition_value_id: i32,
}

#[derive(Queryable, Selectable, Serialize, Debug, Clone, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::location_area_encounter_rates)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
#[pokemonle(tags = ["encounter", "location"])]
pub struct LocationAreaEncounterRate {
    pub location_area_id: i32,
    pub encounter_method_id: i32,
    pub version_id: i32,
    pub rate: i32,
}
