use crate::http::error::AppError;
use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use shared::storage::driver::Driver;
use std::sync::Arc;

pub async fn handler(
    State(driver): State<Arc<dyn Driver>>,
    Path(job_type): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let definitions = driver.get_job_definitions_by_type(&job_type).await?;
    Ok(Json(definitions))
}
