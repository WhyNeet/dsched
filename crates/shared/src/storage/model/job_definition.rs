use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobDefinition {
    pub id: Uuid,
    pub r#type: String,
    pub payload: Json<serde_json::Value>,
    pub schedule_type: String,
    pub schedule: Option<String>,
    pub max_retries: i32,
    pub next_run_at: Option<DateTime<Utc>>,
    pub last_triggered_at: Option<DateTime<Utc>>,
    pub is_enabled: bool,
    pub created_at: DateTime<Utc>,
}
