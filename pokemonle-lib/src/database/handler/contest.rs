use crate::database::schema::{contest_effects, contest_types};
use crate::impl_database_handler;
use crate::model::{ContestEffect, ContestType};

impl_database_handler!(
    ContestTypeHandler,
    ContestType,
    contest_types::dsl::contest_types,
    contest_types::dsl::id
);

impl_database_handler!(
    ContestEffectHandler,
    ContestEffect,
    contest_effects::dsl::contest_effects,
    contest_effects::dsl::id
);
