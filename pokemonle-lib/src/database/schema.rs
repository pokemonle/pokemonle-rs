// @generated automatically by Diesel CLI.

diesel::table! {
    abilities (id) {
        id -> Integer,
        identifier -> Text,
        generation_id -> Integer,
        is_main_series -> Nullable<Bool>,
    }
}

diesel::table! {
    berries (id) {
        id -> Integer,
        item_id -> Integer,
        firmness_id -> Integer,
        natural_gift_power -> Integer,
        natural_gift_type_id -> Integer,
        size -> Integer,
        max_harvest -> Integer,
        growth_time -> Integer,
        soil_dryness -> Integer,
        smoothness -> Integer,
    }
}

diesel::table! {
    berry_firmness (id) {
        id -> Integer,
        identifier -> Text,
    }
}

diesel::table! {
    contest_effects (id) {
        id -> Integer,
        appeal -> Integer,
        jam -> Integer,
    }
}

diesel::table! {
    contest_types (id) {
        id -> Integer,
        identifier -> Text,
    }
}

diesel::table! {
    egg_groups (id) {
        id -> Integer,
        identifier -> Text,
    }
}

diesel::table! {
    evolution_chains (id) {
        id -> Integer,
        baby_trigger_item_id -> Nullable<Integer>,
    }
}

diesel::table! {
    evolution_triggers (id) {
        id -> Integer,
        identifier -> Text,
    }
}

diesel::table! {
    genders (id) {
        id -> Integer,
        identifier -> Text,
    }
}

diesel::table! {
    generations (id) {
        id -> Integer,
        main_region_id -> Integer,
        identifier -> Text,
    }
}

diesel::table! {
    growth_rates (id) {
        id -> Integer,
        identifier -> Text,
        formula -> Text,
    }
}

diesel::table! {
    item_categories (id) {
        id -> Integer,
        pocket_id -> Integer,
        identifier -> Text,
    }
}

diesel::table! {
    item_fling_effects (id) {
        id -> Integer,
        identifier -> Text,
    }
}

diesel::table! {
    item_pockets (id) {
        id -> Integer,
        identifier -> Text,
    }
}

diesel::table! {
    items (id) {
        id -> Integer,
        identifier -> Text,
        category_id -> Integer,
        cost -> Integer,
        fling_power -> Nullable<Integer>,
        fling_effect_id -> Nullable<Integer>,
    }
}

diesel::table! {
    languages (id) {
        id -> Integer,
        iso639 -> Text,
        iso3166 -> Text,
        identifier -> Text,
        official -> Bool,
        order -> Integer,
    }
}

diesel::table! {
    locations (id) {
        id -> Integer,
        region_id -> Nullable<Integer>,
        identifier -> Text,
    }
}

diesel::table! {
    move_damage_classes (id) {
        id -> Integer,
        identifier -> Text,
    }
}

diesel::table! {
    pokemon (id) {
        id -> Integer,
        identifier -> Text,
        species_id -> Integer,
        height -> Integer,
        weight -> Integer,
        base_experience -> Integer,
        order -> Nullable<Integer>,
        is_default -> Bool,
    }
}

diesel::table! {
    pokemon_abilities (pokemon_id, ability_id, slot) {
        pokemon_id -> Integer,
        ability_id -> Integer,
        is_hidden -> Bool,
        slot -> Integer,
    }
}

diesel::table! {
    pokemon_colors (id) {
        id -> Integer,
        identifier -> Text,
    }
}

diesel::table! {
    pokemon_egg_groups (species_id, egg_group_id) {
        species_id -> Integer,
        egg_group_id -> Integer,
    }
}

diesel::table! {
    pokemon_evolution (id) {
        id -> Integer,
        evolved_species_id -> Integer,
        evolution_trigger_id -> Integer,
        trigger_item_id -> Nullable<Integer>,
        minimum_level -> Nullable<Integer>,
        gender_id -> Nullable<Integer>,
        location_id -> Nullable<Integer>,
        held_item_id -> Nullable<Integer>,
        time_of_day -> Nullable<Text>,
        known_move_id -> Nullable<Integer>,
        known_move_type_id -> Nullable<Integer>,
        minimum_happiness -> Nullable<Integer>,
        minimum_beauty -> Nullable<Integer>,
        minimum_affection -> Nullable<Integer>,
        relative_physical_stats -> Nullable<Integer>,
        party_species_id -> Nullable<Integer>,
        party_type_id -> Nullable<Integer>,
        trade_species_id -> Nullable<Integer>,
        needs_overworld_rain -> Nullable<Bool>,
        turn_upside_down -> Nullable<Bool>,
    }
}

diesel::table! {
    pokemon_habitats (id) {
        id -> Integer,
        identifier -> Text,
    }
}

diesel::table! {
    pokemon_shapes (id) {
        id -> Integer,
        identifier -> Text,
    }
}

diesel::table! {
    pokemon_species (id) {
        id -> Integer,
        identifier -> Text,
        generation_id -> Integer,
        evolves_from_species_id -> Nullable<Integer>,
        evolution_chain_id -> Nullable<Integer>,
        color_id -> Integer,
        shape_id -> Integer,
        habitat_id -> Nullable<Integer>,
        gender_rate -> Nullable<Integer>,
        capture_rate -> Nullable<Integer>,
        base_happiness -> Nullable<Integer>,
        is_baby -> Bool,
        hatch_counter -> Integer,
        has_gender_differences -> Bool,
        growth_rate_id -> Integer,
        forms_switchable -> Bool,
        is_legendary -> Bool,
        is_mythical -> Bool,
        order -> Integer,
        conquest_order -> Nullable<Integer>,
    }
}

diesel::table! {
    pokemon_stats (pokemon_id, stat_id) {
        pokemon_id -> Integer,
        stat_id -> Integer,
        base_stat -> Integer,
        effort -> Integer,
    }
}

diesel::table! {
    pokemon_types (pokemon_id, type_id) {
        pokemon_id -> Integer,
        type_id -> Integer,
        slot -> Integer,
    }
}

diesel::table! {
    regions (id) {
        id -> Integer,
        identifier -> Text,
    }
}

diesel::table! {
    types (id) {
        id -> Integer,
        identifier -> Text,
        generation_id -> Integer,
        damage_class_id -> Nullable<Integer>,
    }
}

diesel::table! {
    version_groups (id) {
        id -> Integer,
        identifier -> Text,
        generation_id -> Integer,
        order -> Integer,
    }
}

diesel::table! {
    versions (id) {
        id -> Integer,
        version_group_id -> Integer,
        identifier -> Text,
    }
}

diesel::joinable!(abilities -> generations (generation_id));
diesel::joinable!(berries -> berry_firmness (firmness_id));
diesel::joinable!(berries -> types (natural_gift_type_id));
diesel::joinable!(evolution_chains -> items (baby_trigger_item_id));
diesel::joinable!(item_categories -> item_pockets (pocket_id));
diesel::joinable!(items -> item_categories (category_id));
diesel::joinable!(items -> item_fling_effects (fling_effect_id));
diesel::joinable!(locations -> regions (region_id));
diesel::joinable!(pokemon -> pokemon_species (species_id));
diesel::joinable!(pokemon_abilities -> abilities (ability_id));
diesel::joinable!(pokemon_abilities -> pokemon (pokemon_id));
diesel::joinable!(pokemon_egg_groups -> egg_groups (egg_group_id));
diesel::joinable!(pokemon_egg_groups -> pokemon_species (species_id));
diesel::joinable!(pokemon_evolution -> evolution_triggers (evolution_trigger_id));
diesel::joinable!(pokemon_evolution -> locations (location_id));
diesel::joinable!(pokemon_evolution -> types (party_type_id));
diesel::joinable!(pokemon_species -> evolution_chains (evolution_chain_id));
diesel::joinable!(pokemon_species -> generations (generation_id));
diesel::joinable!(pokemon_species -> growth_rates (growth_rate_id));
diesel::joinable!(pokemon_species -> pokemon_colors (color_id));
diesel::joinable!(pokemon_species -> pokemon_habitats (habitat_id));
diesel::joinable!(pokemon_species -> pokemon_shapes (shape_id));
diesel::joinable!(pokemon_stats -> pokemon (pokemon_id));
diesel::joinable!(pokemon_types -> pokemon (pokemon_id));
diesel::joinable!(pokemon_types -> types (type_id));
diesel::joinable!(types -> generations (generation_id));
diesel::joinable!(types -> move_damage_classes (damage_class_id));
diesel::joinable!(version_groups -> generations (generation_id));
diesel::joinable!(versions -> version_groups (version_group_id));

diesel::allow_tables_to_appear_in_same_query!(
    abilities,
    berries,
    berry_firmness,
    contest_effects,
    contest_types,
    egg_groups,
    evolution_chains,
    evolution_triggers,
    genders,
    generations,
    growth_rates,
    item_categories,
    item_fling_effects,
    item_pockets,
    items,
    languages,
    locations,
    move_damage_classes,
    pokemon,
    pokemon_abilities,
    pokemon_colors,
    pokemon_egg_groups,
    pokemon_evolution,
    pokemon_habitats,
    pokemon_shapes,
    pokemon_species,
    pokemon_stats,
    pokemon_types,
    regions,
    types,
    version_groups,
    versions,
);
