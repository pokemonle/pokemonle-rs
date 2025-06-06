mod docs;
mod error;
mod v1;
use axum::{
    extract::{MatchedPath, Request},
    http::StatusCode,
    response::Redirect,
    Extension,
};

use aide::{
    axum::{routing::get, ApiRouter, IntoApiResponse},
    openapi::{Info, OpenApi},
};

use std::{net::SocketAddr, sync::Arc};
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    aide::generate::on_error(|error| {
        println!("{error}");
    });

    aide::generate::extract_schemas(true);

    let pool = pokemonle_lib::database::DatabaseClient::new().await?;
    let state = v1::AppState { pool };

    let mut api = OpenApi {
        info: Info {
            title: "Pokemonle API".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };

    let app = ApiRouter::new()
        .route("/", get(|| async { Redirect::permanent("/docs") }))
        .nest("/v1", v1::routers())
        .nest("/docs", docs::routes())
        .with_state(state)
        .finish_api(&mut api)
        .layer(Extension(Arc::new(api)))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any), // .allow_credentials(true),
        )
        .layer(
            TraceLayer::new_for_http()
                // Create our own span for the request and include the matched path. The matched
                // path is useful for figuring out which handler the request was routed to.
                .make_span_with(|req: &Request| {
                    let method = req.method();
                    let uri = req.uri();
                    // axum automatically adds this extension.
                    let matched_path = req
                        .extensions()
                        .get::<MatchedPath>()
                        .map(|matched_path| matched_path.as_str());
                    tracing::debug_span!("request", %method, %uri, matched_path)
                })
                // By default `TraceLayer` will log 5xx responses but we're doing our specific
                // logging of errors so disable that
                .on_failure(()),
        )
        .fallback(handler_404);

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(9001);

    // run our app with hyper, listening globally on port 8000
    let listener = tokio::net::TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port))).await?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    Ok(())
}

async fn handler_404() -> impl IntoApiResponse {
    (StatusCode::NOT_FOUND, "404 not found")
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
