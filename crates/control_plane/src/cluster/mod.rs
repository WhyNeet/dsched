use std::net::SocketAddr;

use chrono::{DateTime, Utc};

pub struct ClusterHandle {
    pub key: String,
    pub state: ClusterState,
    // also include a sender for messages
}

pub enum ClusterState {
    Connected {
        address: SocketAddr,
        connected_at: DateTime<Utc>,
    },
    Disconnected {
        last_seen: Option<DateTime<Utc>>,
    },
}
