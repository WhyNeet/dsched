use crate::http::error::AppError;
use axum::{Json, extract::State, response::IntoResponse};
use shared::storage::driver::Driver;
use std::sync::Arc;

pub async fn handler(State(driver): State<Arc<dyn Driver>>) -> Result<impl IntoResponse, AppError> {
    let definitions = driver.get_enabled_job_definitions().await?;
    Ok(Json(definitions))
}
