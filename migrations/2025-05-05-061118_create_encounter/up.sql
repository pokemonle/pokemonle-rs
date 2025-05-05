-- Your SQL goes here

-- 创建遭遇方法表
CREATE TABLE encounter_methods (
    id INTEGER NOT NULL PRIMARY KEY,
    identifier VARCHAR NOT NULL,
    "order" INTEGER NOT NULL
);

-- 创建遭遇槽位表
CREATE TABLE encounter_slots (
    id INTEGER NOT NULL PRIMARY KEY,
    version_group_id INTEGER NOT NULL,
    encounter_method_id INTEGER NOT NULL REFERENCES encounter_methods(id),
    slot INTEGER,
    rarity INTEGER NOT NULL
);

-- 创建遭遇表
CREATE TABLE encounters (
    id INTEGER NOT NULL PRIMARY KEY,
    version_id INTEGER NOT NULL REFERENCES versions(id),
    location_area_id INTEGER NOT NULL REFERENCES location_areas(id),
    encounter_slot_id INTEGER NOT NULL REFERENCES encounter_slots(id),
    pokemon_id INTEGER NOT NULL REFERENCES pokemon(id),
    min_level INTEGER NOT NULL,
    max_level INTEGER NOT NULL
);

-- 创建遭遇条件表
CREATE TABLE encounter_conditions (
    id INTEGER NOT NULL PRIMARY KEY,
    identifier VARCHAR NOT NULL
);

-- 创建遭遇条件值表
CREATE TABLE encounter_condition_values (
    id INTEGER NOT NULL PRIMARY KEY,
    encounter_condition_id INTEGER NOT NULL REFERENCES encounter_conditions(id),
    identifier VARCHAR NOT NULL,
    is_default BOOLEAN NOT NULL
);

-- 创建遭遇条件值映射表
CREATE TABLE encounter_condition_value_map (
    encounter_id INTEGER NOT NULL REFERENCES encounters(id),
    encounter_condition_value_id INTEGER NOT NULL REFERENCES encounter_condition_values(id),
    PRIMARY KEY (encounter_id, encounter_condition_value_id)
);

-- 创建地点区域表
CREATE TABLE location_areas (
    id INTEGER NOT NULL PRIMARY KEY,
    location_id INTEGER NOT NULL,
    game_index INTEGER NOT NULL,
    identifier VARCHAR NOT NULL
);

-- 创建地点区域遭遇率表
CREATE TABLE location_area_encounter_rates (
    location_area_id INTEGER NOT NULL REFERENCES location_areas(id),
    encounter_method_id INTEGER NOT NULL REFERENCES encounter_methods(id),
    version_id INTEGER NOT NULL REFERENCES versions(id),
    rate INTEGER NOT NULL,
    PRIMARY KEY (location_area_id, encounter_method_id, version_id)
);
