use crate::http::error::AppError;
use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use shared::storage::{driver::Driver, model::job_definition::JobDefinition};
use std::sync::Arc;
use uuid::Uuid;

pub async fn handler(
    State(driver): State<Arc<dyn Driver>>,
    Path(id): Path<Uuid>,
    Json(mut definition): Json<JobDefinition>,
) -> Result<impl IntoResponse, AppError> {
    definition.id = id;
    driver.update_job_definition(definition).await?;
    Ok(())
}
