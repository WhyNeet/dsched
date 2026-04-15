use crate::http::{error::AppError, handlers::job_definition::create::JobScheduleDto};
use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use chrono::Utc;
use serde::Deserialize;
use shared::storage::driver::Driver;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdateScheduleRequest {
    pub schedule: JobScheduleDto,
}

pub async fn handler(
    State(driver): State<Arc<dyn Driver>>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateScheduleRequest>,
) -> Result<impl IntoResponse, AppError> {
    let next_run_at = match body.schedule {
        JobScheduleDto::Once(next_run_at) => next_run_at,
        JobScheduleDto::Cron(ref cron) => cron.find_next_occurrence(&Utc::now(), true)?,
        JobScheduleDto::Immediate => Utc::now(),
    };
    let schedule_type = match body.schedule {
        JobScheduleDto::Immediate => "immediate",
        JobScheduleDto::Once(_) => "once",
        JobScheduleDto::Cron(_) => "cron",
    };
    let schedule = match body.schedule {
        JobScheduleDto::Cron(cron) => Some(cron.to_string()),
        _ => None,
    };

    driver
        .update_job_definition_schedule(id, schedule_type.to_string(), schedule, next_run_at)
        .await?;
    Ok(())
}
