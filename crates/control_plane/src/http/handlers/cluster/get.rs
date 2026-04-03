use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};

use crate::{http::error::AppError, storage::driver::Driver};

pub async fn handler(
    State(driver): State<Arc<dyn Driver>>,
    Path(key): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let clusters = driver.get_clusters_by_key(key).await?;

    Ok(Json(clusters))
}
