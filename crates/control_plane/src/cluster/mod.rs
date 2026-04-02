use std::net::SocketAddr;

use chrono::{DateTime, Utc};

pub mod registry;
pub mod tcp;

#[derive(Debug, Clone)]
pub struct ClusterHandle {
    pub key: String,
    pub state: ClusterState,
    pub sender: flume::Sender<ClusterMessage>,
}

#[derive(Debug, Clone)]
pub enum ClusterState {
    Connected {
        address: SocketAddr,
        connected_at: DateTime<Utc>,
    },
    Disconnected {
        last_seen: Option<DateTime<Utc>>,
    },
}

pub enum ClusterMessage {}
