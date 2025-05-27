mod ability;
mod contest;
mod evo;
mod item;
mod language;
mod r#move;
// mod openapi;
mod location;
mod pokemon;
mod router;

use aide::axum::{routing::get, ApiRouter, IntoApiResponse};
use axum::{http::StatusCode, Json};
use router::ResourceRouter;
use serde_json::json;

use pokemonle_lib::database::DatabaseClient;

#[derive(Clone)]
pub struct AppState {
    pub pool: DatabaseClient,
}

pub fn routers() -> ApiRouter<AppState> {
    use pokemonle_lib::database::entity::prelude::*;

    ApiRouter::new()
        .route("/", get(get_into))
        .nest("/abilities", ability::routers())
        .nest("/berries", Berries::routers_with(|op| op.tag("berry")))
        .nest(
            "/berry-firmness",
            BerryFirmness::routers_with(|op| op.tag("berry")),
        )
        .merge(contest::routers())
        .nest(
            "/encounters",
            Encounters::routers_with(|op| op.tag("encounter")),
        )
        .merge(evo::routers())
        .nest("/items", item::routers())
        .nest(
            "/item-categories",
            ItemCategories::routers_with(|op| op.tag("item")),
        )
        .nest(
            "/item-pockets",
            ItemPockets::routers_with(|op| op.tag("item")),
        )
        .merge(language::routers())
        .merge(location::routers())
        .nest("/moves", r#move::routers())
        .merge(pokemon::routers())
}

async fn get_into() -> impl IntoApiResponse {
    let git = {
        let branch = env!("VERGEN_GIT_BRANCH");
        let commit = env!("VERGEN_GIT_SHA");
        let version = env!("VERGEN_GIT_DESCRIBE");
        let commit_timestamp = env!("VERGEN_GIT_COMMIT_TIMESTAMP");

        json!({
            "branch": branch,
            "commit": json!({
                "sha": commit,
                "timestamp": commit_timestamp,
            }),
            "version": version,
        })
    };

    let rustc = {
        let channel = env!("VERGEN_RUSTC_CHANNEL");
        let version = env!("VERGEN_RUSTC_SEMVER");

        json!({
            "channel": channel,
            "version": version,
        })
    };

    let resp = json!({
        "git": git,
        "rustc": rustc,
    });

    (StatusCode::OK, Json(resp))
}
