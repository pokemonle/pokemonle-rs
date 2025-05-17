use crate::database::entry::{
    abilities::Entity as Abilities,
    ability_names::{Column::LocalLanguageId, Entity as AbilityNames},
};
use crate::localized_resource_handler;

localized_resource_handler!(Abilities, AbilityNames, LocalLanguageId);
