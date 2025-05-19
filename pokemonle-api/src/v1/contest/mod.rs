use aide::axum::ApiRouter;

use pokemonle_lib::database::entity::prelude::*;

use super::{router::ResourceRouter, AppState};

pub fn routers() -> ApiRouter<AppState> {
    ApiRouter::new()
        .nest(
            "/contest-types",
            ContestTypes::routers_with(|op| op.tag("contest")),
        )
        .nest(
            "/contest-effects",
            ContestEffects::routers_with(|op| op.tag("contest")),
        )
}
