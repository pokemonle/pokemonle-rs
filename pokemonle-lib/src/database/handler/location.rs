use crate::database::schema::{location_areas, location_names, locations, region_names, regions};
use crate::model::{Location, LocationArea, Region};
use crate::{impl_database_handler, impl_database_locale_handler};

impl_database_handler!(
    LocationHandler,
    Location,
    locations::dsl::locations,
    locations::dsl::id
);

impl_database_handler!(
    LocationAreaHandler,
    LocationArea,
    location_areas::dsl::location_areas,
    location_areas::dsl::id
);

impl_database_handler!(
    RegionHandler,
    Region,
    regions::dsl::regions,
    regions::dsl::id
);

impl_database_locale_handler!(
    LocationHandler,
    Location,
    locations::dsl::locations,
    locations::dsl::id,
    location_names::dsl::location_names,
    location_names::dsl::location_id,
    location_names::dsl::name,
    location_names::dsl::local_language_id
);

impl_database_locale_handler!(
    RegionHandler,
    Region,
    regions::dsl::regions,
    regions::dsl::id,
    region_names::dsl::region_names,
    region_names::dsl::region_id,
    region_names::dsl::name,
    region_names::dsl::local_language_id
);
