use crate::http::error::AppError;
use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};
use shared::storage::driver::Driver;
use std::sync::Arc;

#[derive(serde::Deserialize)]
pub struct ListParams {
    limit: Option<u32>,
    offset: Option<u32>,
}

impl Default for ListParams {
    fn default() -> Self {
        Self {
            limit: Some(50),
            offset: Some(0),
        }
    }
}

pub async fn handler(
    State(driver): State<Arc<dyn Driver>>,
    Query(params): Query<ListParams>,
) -> Result<impl IntoResponse, AppError> {
    let limit = params.limit.unwrap_or(50);
    let offset = params.offset.unwrap_or(0);

    let definitions = driver.list_job_definitions(limit, offset).await?;
    Ok(Json(definitions))
}
