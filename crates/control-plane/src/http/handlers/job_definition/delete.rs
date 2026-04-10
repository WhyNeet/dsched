use crate::http::error::AppError;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use shared::storage::driver::Driver;
use std::sync::Arc;
use uuid::Uuid;

pub async fn handler(
    State(driver): State<Arc<dyn Driver>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    driver.delete_job_definition(id).await?;
    Ok(())
}
