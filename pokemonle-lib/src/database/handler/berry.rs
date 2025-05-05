use crate::database::schema::{berries, berry_firmness};
use crate::impl_database_handler;
use crate::model::{Berry, BerryFirmness};

impl_database_handler!(BerryHandler, Berry, berries::dsl::berries, berries::dsl::id);

impl_database_handler!(
    BerryFirmnessHandler,
    BerryFirmness,
    berry_firmness::dsl::berry_firmness,
    berry_firmness::dsl::id
);
