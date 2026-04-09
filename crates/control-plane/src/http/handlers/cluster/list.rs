use std::sync::Arc;

use crate::http::error::AppError;
use axum::{Json, extract::State, response::IntoResponse};
use shared::storage::driver::Driver;

pub async fn handler(State(driver): State<Arc<dyn Driver>>) -> Result<impl IntoResponse, AppError> {
    let clusters = driver.list_distinct_cluster_keys().await?;

    Ok(Json(clusters))
}
