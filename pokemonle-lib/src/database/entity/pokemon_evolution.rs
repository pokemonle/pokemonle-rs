//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.11

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    Eq,
    Serialize,
    Deserialize,
    schemars :: JsonSchema,
    aide :: OperationIo,
)]
#[sea_orm(table_name = "pokemon_evolution")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub evolved_species_id: i32,
    pub evolution_trigger_id: i32,
    pub trigger_item_id: Option<i32>,
    pub minimum_level: Option<i32>,
    pub gender_id: Option<i32>,
    pub location_id: Option<i32>,
    pub held_item_id: Option<i32>,
    pub time_of_day: Option<String>,
    pub known_move_id: Option<i32>,
    pub known_move_type_id: Option<i32>,
    pub minimum_happiness: Option<i32>,
    pub minimum_beauty: Option<i32>,
    pub minimum_affection: Option<i32>,
    pub relative_physical_stats: Option<i32>,
    pub party_species_id: Option<i32>,
    pub party_type_id: Option<i32>,
    pub trade_species_id: Option<i32>,
    pub needs_overworld_rain: Option<bool>,
    pub turn_upside_down: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::evolution_triggers::Entity",
        from = "Column::EvolutionTriggerId",
        to = "super::evolution_triggers::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    EvolutionTriggers,
    #[sea_orm(
        belongs_to = "super::items::Entity",
        from = "Column::TriggerItemId",
        to = "super::items::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Items2,
    #[sea_orm(
        belongs_to = "super::items::Entity",
        from = "Column::HeldItemId",
        to = "super::items::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Items1,
    #[sea_orm(
        belongs_to = "super::locations::Entity",
        from = "Column::LocationId",
        to = "super::locations::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Locations,
    #[sea_orm(
        belongs_to = "super::pokemon_species::Entity",
        from = "Column::TradeSpeciesId",
        to = "super::pokemon_species::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    PokemonSpecies3,
    #[sea_orm(
        belongs_to = "super::pokemon_species::Entity",
        from = "Column::PartySpeciesId",
        to = "super::pokemon_species::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    PokemonSpecies2,
    #[sea_orm(
        belongs_to = "super::pokemon_species::Entity",
        from = "Column::EvolvedSpeciesId",
        to = "super::pokemon_species::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    PokemonSpecies1,
    #[sea_orm(
        belongs_to = "super::types::Entity",
        from = "Column::PartyTypeId",
        to = "super::types::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Types,
}

impl Related<super::evolution_triggers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EvolutionTriggers.def()
    }
}

impl Related<super::locations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Locations.def()
    }
}

impl Related<super::types::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Types.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
