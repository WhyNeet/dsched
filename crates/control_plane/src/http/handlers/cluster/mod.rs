use axum::{Router, routing};

use crate::http::state::AppState;

mod create;
mod get;
mod list;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", routing::post(create::handler).get(list::handler))
        .route("/{id}", routing::get(get::handler))
}
