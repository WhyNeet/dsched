use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Cluster {
    pub key: String,
    pub status: ClusterStatus,
    pub last_seen: Option<DateTime<Utc>>,
    pub connected_at: Option<DateTime<Utc>>,
    pub address: Option<String>,
}

#[derive(Debug, sqlx::Type, Serialize)]
#[sqlx(type_name = "CLUSTER_STATUS", rename_all = "lowercase")]
pub enum ClusterStatus {
    Disconnected,
    Connected,
}
