use std::sync::Arc;

use axum::extract::FromRef;

use crate::storage::driver::Driver;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub driver: Arc<dyn Driver>,
}
