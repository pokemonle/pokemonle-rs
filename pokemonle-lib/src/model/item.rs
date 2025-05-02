use crate::database::schema;
use aide::OperationIo;
use diesel::prelude::*;

use pokemonle_trait::StructName;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::item_pockets)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
pub struct ItemPocket {
    pub id: i32,
    pub identifier: String,
}

#[derive(Queryable, Selectable, Serialize, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::item_categories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
pub struct ItemCategory {
    pub id: i32,
    pub pocket_id: i32,
    pub identifier: String,
}

#[derive(Queryable, Selectable, Serialize, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::item_fling_effects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
pub struct ItemFlingEffect {
    pub id: i32,
    pub identifier: String,
}

#[derive(Queryable, Selectable, Serialize, JsonSchema, StructName, OperationIo)]
#[diesel(table_name = schema::items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite, diesel::pg::Pg))]
pub struct Item {
    pub id: i32,
    pub identifier: String,
    pub category_id: i32,
    pub cost: i32,
    pub fling_effect_id: Option<i32>,
    pub fling_power: Option<i32>,
}
