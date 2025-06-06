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
#[sea_orm(table_name = "encounter_condition_values")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub encounter_condition_id: i32,
    pub identifier: String,
    pub is_default: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::encounter_condition_value_map::Entity")]
    EncounterConditionValueMap,
    #[sea_orm(
        belongs_to = "super::encounter_conditions::Entity",
        from = "Column::EncounterConditionId",
        to = "super::encounter_conditions::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    EncounterConditions,
}

impl Related<super::encounter_condition_value_map::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EncounterConditionValueMap.def()
    }
}

impl Related<super::encounter_conditions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EncounterConditions.def()
    }
}

impl Related<super::encounters::Entity> for Entity {
    fn to() -> RelationDef {
        super::encounter_condition_value_map::Relation::Encounters.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::encounter_condition_value_map::Relation::EncounterConditionValues
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
