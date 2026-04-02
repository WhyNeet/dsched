use std::sync::Arc;

use anyhow::Context;
use axum::Router;

use crate::config::Config;
use crate::http::state::AppState;
use crate::storage::driver::Driver;

mod error;
mod handlers;
mod state;

pub async fn run(config: Arc<Config>, driver: impl Driver + 'static) -> anyhow::Result<()> {
    let state = AppState {
        driver: Arc::new(driver),
    };

    let app = Router::new().merge(handlers::router()).with_state(state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", config.http_port))
        .await
        .unwrap();

    tracing::info!("listening on port {}", config.http_port);

    axum::serve(listener, app)
        .await
        .context("failed to start server")
}
