use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Node {
    pub id: Uuid,
    pub cluster_key: String,
    pub last_seen: DateTime<Utc>,
}
