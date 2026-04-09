use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::http::error::AppError;
use shared::storage::driver::Driver;

#[derive(Deserialize)]
pub struct ListClustersQuery {
    offset: i64,
    limit: i64,
}

pub async fn handler(
    State(driver): State<Arc<dyn Driver>>,
    Query(query): Query<ListClustersQuery>,
) -> Result<impl IntoResponse, AppError> {
    let clusters = driver.list_clusters(query.offset, query.limit).await?;

    Ok(Json(clusters))
}
