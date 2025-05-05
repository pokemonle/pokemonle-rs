use crate::database::schema::{
    encounter_condition_values, encounter_conditions, encounter_methods, encounter_slots,
    encounters,
};
use crate::impl_database_handler;
use crate::model::{
    Encounter, EncounterCondition, EncounterConditionValue, EncounterMethod, EncounterSlot,
};

impl_database_handler!(
    EncounterHandler,
    Encounter,
    encounters::dsl::encounters,
    encounters::dsl::id
);

impl_database_handler!(
    EncounterConditionHandler,
    EncounterCondition,
    encounter_conditions::dsl::encounter_conditions,
    encounter_conditions::dsl::id
);

impl_database_handler!(
    EncounterConditionValueHandler,
    EncounterConditionValue,
    encounter_condition_values::dsl::encounter_condition_values,
    encounter_condition_values::dsl::id
);

impl_database_handler!(
    EncounterSlotHandler,
    EncounterSlot,
    encounter_slots::dsl::encounter_slots,
    encounter_slots::dsl::id
);

impl_database_handler!(
    EncounterMethodHandler,
    EncounterMethod,
    encounter_methods::dsl::encounter_methods,
    encounter_methods::dsl::id
);
