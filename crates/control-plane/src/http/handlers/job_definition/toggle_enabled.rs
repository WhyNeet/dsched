use crate::http::error::AppError;
use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use serde::Deserialize;
use shared::storage::driver::Driver;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ToggleEnabledRequest {
    pub enabled: bool,
}

pub async fn handler(
    State(driver): State<Arc<dyn Driver>>,
    Path(id): Path<Uuid>,
    Json(body): Json<ToggleEnabledRequest>,
) -> Result<impl IntoResponse, AppError> {
    driver
        .toggle_job_definition_enabled(id, body.enabled)
        .await?;
    Ok(())
}
