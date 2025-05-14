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
    ability_flavor_text (ability_id, version_group_id, language_id) {
        ability_id -> Integer,
        version_group_id -> Integer,
        language_id -> Integer,
        flavor_text -> Text,
    }
}

diesel::table! {
    ability_names (ability_id, local_language_id) {
        ability_id -> Integer,
        local_language_id -> Integer,
        name -> Text,
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
    berry_firmness_names (berry_firmness_id, local_language_id) {
        berry_firmness_id -> Integer,
        local_language_id -> Integer,
        name -> Text,
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
    encounter_condition_value_map (encounter_id, encounter_condition_value_id) {
        encounter_id -> Integer,
        encounter_condition_value_id -> Integer,
    }
}

diesel::table! {
    encounter_condition_values (id) {
        id -> Integer,
        encounter_condition_id -> Integer,
        identifier -> Text,
        is_default -> Bool,
    }
}

diesel::table! {
    encounter_conditions (id) {
        id -> Integer,
        identifier -> Text,
    }
}

diesel::table! {
    encounter_methods (id) {
        id -> Integer,
        identifier -> Text,
        order -> Integer,
    }
}

diesel::table! {
    encounter_slots (id) {
        id -> Integer,
        version_group_id -> Integer,
        encounter_method_id -> Integer,
        slot -> Nullable<Integer>,
        rarity -> Integer,
    }
}

diesel::table! {
    encounters (id) {
        id -> Integer,
        version_id -> Integer,
        location_area_id -> Integer,
        encounter_slot_id -> Integer,
        pokemon_id -> Integer,
        min_level -> Integer,
        max_level -> Integer,
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
    generation_names (generation_id, local_language_id) {
        generation_id -> Integer,
        local_language_id -> Integer,
        name -> Text,
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
    item_flavor_text (item_id, version_group_id, language_id) {
        item_id -> Integer,
        version_group_id -> Integer,
        language_id -> Integer,
        flavor_text -> Text,
    }
}

diesel::table! {
    item_fling_effects (id) {
        id -> Integer,
        identifier -> Text,
    }
}

diesel::table! {
    item_names (item_id, local_language_id) {
        item_id -> Integer,
        local_language_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    item_pocket_names (item_pocket_id, local_language_id) {
        item_pocket_id -> Integer,
        local_language_id -> Integer,
        name -> Text,
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
    language_names (language_id, local_language_id) {
        language_id -> Integer,
        local_language_id -> Integer,
        name -> Text,
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
    location_area_encounter_rates (location_area_id, encounter_method_id, version_id) {
        location_area_id -> Integer,
        encounter_method_id -> Integer,
        version_id -> Integer,
        rate -> Integer,
    }
}

diesel::table! {
    location_areas (id) {
        id -> Integer,
        location_id -> Integer,
        game_index -> Integer,
        identifier -> Text,
    }
}

diesel::table! {
    location_names (location_id, local_language_id) {
        location_id -> Integer,
        local_language_id -> Integer,
        name -> Text,
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
    move_effects (id) {
        id -> Integer,
    }
}

diesel::table! {
    move_flavor_text (move_id, version_group_id, language_id) {
        move_id -> Integer,
        version_group_id -> Integer,
        language_id -> Integer,
        flavor_text -> Text,
    }
}

diesel::table! {
    move_names (move_id, local_language_id) {
        move_id -> Integer,
        local_language_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    move_targets (id) {
        id -> Integer,
        identifier -> Text,
    }
}

diesel::table! {
    moves (id) {
        id -> Integer,
        identifier -> Text,
        generation_id -> Integer,
        type_id -> Nullable<Integer>,
        power -> Nullable<Integer>,
        pp -> Nullable<Integer>,
        accuracy -> Nullable<Integer>,
        priority -> Integer,
        target_id -> Integer,
        damage_class_id -> Integer,
        effect_id -> Nullable<Integer>,
        effect_chance -> Nullable<Integer>,
        contest_type_id -> Nullable<Integer>,
        contest_effect_id -> Nullable<Integer>,
    }
}

diesel::table! {
    pokedex_version_groups (pokedex_id, version_group_id) {
        pokedex_id -> Integer,
        version_group_id -> Integer,
    }
}

diesel::table! {
    pokedexes (id) {
        id -> Integer,
        region_id -> Nullable<Integer>,
        identifier -> Text,
        is_main_series -> Bool,
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
    pokemon_color_names (pokemon_color_id, local_language_id) {
        pokemon_color_id -> Integer,
        local_language_id -> Integer,
        name -> Text,
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
    pokemon_move_methods (id) {
        id -> Integer,
        identifier -> Text,
    }
}

diesel::table! {
    pokemon_moves (pokemon_id, version_group_id, move_id, pokemon_move_method_id, level) {
        pokemon_id -> Integer,
        version_group_id -> Integer,
        move_id -> Integer,
        pokemon_move_method_id -> Integer,
        level -> Integer,
        order -> Nullable<Integer>,
        mastery -> Nullable<Integer>,
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
    pokemon_species_flavor_text (species_id, version_id, language_id) {
        species_id -> Integer,
        version_id -> Integer,
        language_id -> Integer,
        flavor_text -> Text,
    }
}

diesel::table! {
    pokemon_species_names (pokemon_species_id, local_language_id) {
        pokemon_species_id -> Integer,
        local_language_id -> Integer,
        name -> Text,
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
    region_names (region_id, local_language_id) {
        region_id -> Integer,
        local_language_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    regions (id) {
        id -> Integer,
        identifier -> Text,
    }
}

diesel::table! {
    type_names (type_id, local_language_id) {
        type_id -> Integer,
        local_language_id -> Integer,
        name -> Text,
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
    version_group_names (version_group_id, local_language_id) {
        version_group_id -> Integer,
        local_language_id -> Integer,
        name -> Text,
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
    version_names (version_id, local_language_id) {
        version_id -> Integer,
        local_language_id -> Integer,
        name -> Text,
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
diesel::joinable!(ability_flavor_text -> abilities (ability_id));
diesel::joinable!(ability_flavor_text -> languages (language_id));
diesel::joinable!(ability_flavor_text -> version_groups (version_group_id));
diesel::joinable!(ability_names -> abilities (ability_id));
diesel::joinable!(ability_names -> languages (local_language_id));
diesel::joinable!(berries -> berry_firmness (firmness_id));
diesel::joinable!(berries -> types (natural_gift_type_id));
diesel::joinable!(berry_firmness_names -> languages (local_language_id));
diesel::joinable!(encounter_condition_value_map -> encounter_condition_values (encounter_condition_value_id));
diesel::joinable!(encounter_condition_value_map -> encounters (encounter_id));
diesel::joinable!(encounter_condition_values -> encounter_conditions (encounter_condition_id));
diesel::joinable!(encounter_slots -> encounter_methods (encounter_method_id));
diesel::joinable!(encounters -> encounter_slots (encounter_slot_id));
diesel::joinable!(encounters -> location_areas (location_area_id));
diesel::joinable!(encounters -> pokemon (pokemon_id));
diesel::joinable!(encounters -> versions (version_id));
diesel::joinable!(evolution_chains -> items (baby_trigger_item_id));
diesel::joinable!(generation_names -> generations (generation_id));
diesel::joinable!(generation_names -> languages (local_language_id));
diesel::joinable!(item_categories -> item_pockets (pocket_id));
diesel::joinable!(item_flavor_text -> items (item_id));
diesel::joinable!(item_flavor_text -> languages (language_id));
diesel::joinable!(item_flavor_text -> version_groups (version_group_id));
diesel::joinable!(item_names -> items (item_id));
diesel::joinable!(item_names -> languages (local_language_id));
diesel::joinable!(item_pocket_names -> item_pockets (item_pocket_id));
diesel::joinable!(item_pocket_names -> languages (local_language_id));
diesel::joinable!(items -> item_categories (category_id));
diesel::joinable!(items -> item_fling_effects (fling_effect_id));
diesel::joinable!(location_area_encounter_rates -> encounter_methods (encounter_method_id));
diesel::joinable!(location_area_encounter_rates -> location_areas (location_area_id));
diesel::joinable!(location_area_encounter_rates -> versions (version_id));
diesel::joinable!(location_areas -> locations (location_id));
diesel::joinable!(location_names -> languages (local_language_id));
diesel::joinable!(location_names -> locations (location_id));
diesel::joinable!(locations -> regions (region_id));
diesel::joinable!(move_flavor_text -> languages (language_id));
diesel::joinable!(move_flavor_text -> moves (move_id));
diesel::joinable!(move_flavor_text -> version_groups (version_group_id));
diesel::joinable!(move_names -> languages (local_language_id));
diesel::joinable!(move_names -> moves (move_id));
diesel::joinable!(moves -> move_damage_classes (damage_class_id));
diesel::joinable!(moves -> move_effects (effect_id));
diesel::joinable!(moves -> move_targets (target_id));
diesel::joinable!(pokedex_version_groups -> pokedexes (pokedex_id));
diesel::joinable!(pokedex_version_groups -> version_groups (version_group_id));
diesel::joinable!(pokedexes -> regions (region_id));
diesel::joinable!(pokemon -> pokemon_species (species_id));
diesel::joinable!(pokemon_abilities -> abilities (ability_id));
diesel::joinable!(pokemon_abilities -> pokemon (pokemon_id));
diesel::joinable!(pokemon_color_names -> languages (local_language_id));
diesel::joinable!(pokemon_color_names -> pokemon_colors (pokemon_color_id));
diesel::joinable!(pokemon_egg_groups -> egg_groups (egg_group_id));
diesel::joinable!(pokemon_egg_groups -> pokemon_species (species_id));
diesel::joinable!(pokemon_evolution -> evolution_triggers (evolution_trigger_id));
diesel::joinable!(pokemon_evolution -> locations (location_id));
diesel::joinable!(pokemon_evolution -> types (party_type_id));
diesel::joinable!(pokemon_moves -> moves (move_id));
diesel::joinable!(pokemon_moves -> pokemon (pokemon_id));
diesel::joinable!(pokemon_moves -> pokemon_move_methods (pokemon_move_method_id));
diesel::joinable!(pokemon_moves -> version_groups (version_group_id));
diesel::joinable!(pokemon_species -> evolution_chains (evolution_chain_id));
diesel::joinable!(pokemon_species -> generations (generation_id));
diesel::joinable!(pokemon_species -> growth_rates (growth_rate_id));
diesel::joinable!(pokemon_species -> pokemon_colors (color_id));
diesel::joinable!(pokemon_species -> pokemon_habitats (habitat_id));
diesel::joinable!(pokemon_species -> pokemon_shapes (shape_id));
diesel::joinable!(pokemon_species_flavor_text -> languages (language_id));
diesel::joinable!(pokemon_species_flavor_text -> pokemon_species (species_id));
diesel::joinable!(pokemon_species_flavor_text -> versions (version_id));
diesel::joinable!(pokemon_species_names -> languages (local_language_id));
diesel::joinable!(pokemon_species_names -> pokemon_species (pokemon_species_id));
diesel::joinable!(pokemon_stats -> pokemon (pokemon_id));
diesel::joinable!(pokemon_types -> pokemon (pokemon_id));
diesel::joinable!(pokemon_types -> types (type_id));
diesel::joinable!(region_names -> languages (local_language_id));
diesel::joinable!(region_names -> regions (region_id));
diesel::joinable!(type_names -> languages (local_language_id));
diesel::joinable!(type_names -> types (type_id));
diesel::joinable!(types -> generations (generation_id));
diesel::joinable!(types -> move_damage_classes (damage_class_id));
diesel::joinable!(version_group_names -> languages (local_language_id));
diesel::joinable!(version_group_names -> version_groups (version_group_id));
diesel::joinable!(version_groups -> generations (generation_id));
diesel::joinable!(version_names -> languages (local_language_id));
diesel::joinable!(version_names -> versions (version_id));
diesel::joinable!(versions -> version_groups (version_group_id));

diesel::allow_tables_to_appear_in_same_query!(
    abilities,
    ability_flavor_text,
    ability_names,
    berries,
    berry_firmness,
    berry_firmness_names,
    contest_effects,
    contest_types,
    egg_groups,
    encounter_condition_value_map,
    encounter_condition_values,
    encounter_conditions,
    encounter_methods,
    encounter_slots,
    encounters,
    evolution_chains,
    evolution_triggers,
    genders,
    generation_names,
    generations,
    growth_rates,
    item_categories,
    item_flavor_text,
    item_fling_effects,
    item_names,
    item_pocket_names,
    item_pockets,
    items,
    language_names,
    languages,
    location_area_encounter_rates,
    location_areas,
    location_names,
    locations,
    move_damage_classes,
    move_effects,
    move_flavor_text,
    move_names,
    move_targets,
    moves,
    pokedex_version_groups,
    pokedexes,
    pokemon,
    pokemon_abilities,
    pokemon_color_names,
    pokemon_colors,
    pokemon_egg_groups,
    pokemon_evolution,
    pokemon_habitats,
    pokemon_move_methods,
    pokemon_moves,
    pokemon_shapes,
    pokemon_species,
    pokemon_species_flavor_text,
    pokemon_species_names,
    pokemon_stats,
    pokemon_types,
    region_names,
    regions,
    type_names,
    types,
    version_group_names,
    version_groups,
    version_names,
    versions,
);
