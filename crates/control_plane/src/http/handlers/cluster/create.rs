use std::sync::Arc;

use axum::{Json, extract::State, response::IntoResponse};
use serde::Deserialize;

use crate::{http::error::AppError, storage::driver::Driver};

#[derive(Deserialize)]
pub struct CreateClusterDto {
    key: String,
}

pub async fn handler(
    State(driver): State<Arc<dyn Driver>>,
    Json(dto): Json<CreateClusterDto>,
) -> Result<impl IntoResponse, AppError> {
    let cluster = driver.create_cluster(dto.key).await?;

    Ok(Json(cluster))
}
