use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Job {
    pub id: Uuid,
    pub r#type: String,
    pub payload: sqlx::types::Json<serde_json::Value>,
    pub run_at: Option<DateTime<Utc>>,
    pub status: JobStatus,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "lowercase")]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
}
