use crate::http::state::AppState;
use axum::{Router, routing};

mod create;
mod delete;
mod get;
mod get_by_type;
mod get_enabled;
mod list;
mod toggle_enabled;
mod update;
mod update_schedule;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", routing::get(list::handler).post(create::handler))
        .route("/enabled", routing::get(get_enabled::handler))
        .route("/type/{job_type}", routing::get(get_by_type::handler))
        .route(
            "/{id}",
            routing::get(get::handler)
                .put(update::handler)
                .delete(delete::handler),
        )
        .route("/{id}/schedule", routing::patch(update_schedule::handler))
        .route("/{id}/enabled", routing::patch(toggle_enabled::handler))
}
