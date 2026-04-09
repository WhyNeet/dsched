use std::sync::Arc;

use axum::extract::FromRef;

use shared::storage::driver::Driver;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub driver: Arc<dyn Driver>,
}
