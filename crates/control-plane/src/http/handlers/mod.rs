use axum::{Router, routing};

use crate::http::state::AppState;

mod cluster;
mod job;
mod job_definition;

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", routing::get(root))
        .nest("/clusters", cluster::router())
        .nest("/job-definitions", job_definition::router())
        .nest("/jobs", job::router())
}
