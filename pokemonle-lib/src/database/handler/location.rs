use crate::database::schema::{location_areas, locations, regions};
use crate::impl_database_handler;
use crate::model::{Location, LocationArea, Region};

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
