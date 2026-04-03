use cluster_proto::message::ClusterMessage;
use uuid::Uuid;

pub mod registry;
pub mod tcp;

#[derive(Debug, Clone)]
pub struct ClusterHandle {
    pub id: Uuid,
    pub key: String,
    pub sender: flume::Sender<ClusterMessage>,
}

impl ClusterHandle {
    pub fn new(id: Uuid, key: String) -> (ClusterHandle, flume::Receiver<ClusterMessage>) {
        let (tx, rx) = flume::unbounded();

        (
            Self {
                id,
                key,
                sender: tx,
            },
            rx,
        )
    }
}
