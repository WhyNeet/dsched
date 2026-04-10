use crate::http::error::AppError;
use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use shared::storage::driver::Driver;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdateScheduleRequest {
    pub schedule: Option<String>,
    pub next_run_at: Option<DateTime<Utc>>,
}

pub async fn handler(
    State(driver): State<Arc<dyn Driver>>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateScheduleRequest>,
) -> Result<impl IntoResponse, AppError> {
    driver
        .update_job_definition_schedule(id, body.schedule, body.next_run_at)
        .await?;
    Ok(())
}
