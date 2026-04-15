use crate::http::error::AppError;
use axum::{Json, extract::State, response::IntoResponse};
use chrono::{DateTime, Utc};
use croner::Cron;
use serde::Deserialize;
use shared::storage::{driver::Driver, model::job_definition::JobDefinition};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct JobDefinitionDto {
    pub r#type: String,
    pub payload: serde_json::Value,
    pub schedule: JobScheduleDto,
    pub max_retries: i32,
    pub is_enabled: bool,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum JobScheduleDto {
    Immediate,
    Once(DateTime<Utc>),
    Cron(Cron),
}

pub async fn handler(
    State(driver): State<Arc<dyn Driver>>,
    Json(definition): Json<JobDefinitionDto>,
) -> Result<impl IntoResponse, AppError> {
    driver
        .create_job_definition(JobDefinition {
            id: Uuid::new_v4(),
            r#type: definition.r#type,
            payload: definition.payload.into(),
            schedule_type: match definition.schedule {
                JobScheduleDto::Immediate => "immediate",
                JobScheduleDto::Once(_) => "once",
                JobScheduleDto::Cron(_) => "cron",
            }
            .to_string(),
            schedule: match definition.schedule {
                JobScheduleDto::Cron(ref cron) => Some(cron.to_string()),
                _ => None,
            },
            max_retries: definition.max_retries,
            next_run_at: match definition.schedule {
                JobScheduleDto::Once(next_run_at) => next_run_at,
                JobScheduleDto::Cron(cron) => cron.find_next_occurrence(&Utc::now(), true)?,
                JobScheduleDto::Immediate => Utc::now(),
            },
            last_triggered_at: None,
            is_enabled: definition.is_enabled,
            created_at: Utc::now(),
        })
        .await?;
    Ok(())
}
