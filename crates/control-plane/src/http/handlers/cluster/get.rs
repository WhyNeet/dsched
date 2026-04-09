use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use shared::storage::driver::Driver;

use crate::http::error::AppError;

pub async fn handler(
    State(driver): State<Arc<dyn Driver>>,
    Path(key): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let clusters = driver.count_nodes_by_cluster_key(&key).await?;

    Ok(Json(clusters))
}
