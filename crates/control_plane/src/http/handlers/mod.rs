use axum::{Router, routing};

use crate::http::state::AppState;

// mod cluster;

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", routing::get(root))
}
