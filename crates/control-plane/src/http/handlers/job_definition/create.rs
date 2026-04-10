use crate::http::error::AppError;
use axum::{Json, extract::State, response::IntoResponse};
use shared::storage::{driver::Driver, model::job_definition::JobDefinition};
use std::sync::Arc;

pub async fn handler(
    State(driver): State<Arc<dyn Driver>>,
    Json(definition): Json<JobDefinition>,
) -> Result<impl IntoResponse, AppError> {
    driver.create_job_definition(definition).await?;
    Ok(())
}
