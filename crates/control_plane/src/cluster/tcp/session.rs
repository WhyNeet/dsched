use std::{net::SocketAddr, sync::Arc};

use cluster_proto::message::ClusterMessage;
use uuid::Uuid;

use crate::{
    cluster::{ClusterHandle, registry::ClusterRegistry},
    storage::{driver::Driver, model::cluster::ClusterStatus},
    util,
};

#[derive(Debug, Default)]
pub enum ClusterSessionState {
    #[default]
    WaitingForAuthentication,
    Ready {
        id: Uuid,
        key: String,
    },
    Closed,
}

pub struct ClusterSession {
    state: ClusterSessionState,
    driver: Arc<dyn Driver>,
    addr: SocketAddr,
    registry: ClusterRegistry,
    pub receiver: Option<flume::Receiver<ClusterMessage>>,
}

impl ClusterSession {
    pub fn new(driver: Arc<dyn Driver>, addr: SocketAddr, registry: ClusterRegistry) -> Self {
        Self {
            state: Default::default(),
            driver,
            addr,
            registry,
            receiver: None,
        }
    }

    pub async fn handle_message(&mut self, msg: &ClusterMessage) -> anyhow::Result<()> {
        match (&self.state, msg) {
            (
                ClusterSessionState::WaitingForAuthentication,
                ClusterMessage::ClusterAuth(id, key),
            ) => {
                let key = key.to_string();
                self.state = ClusterSessionState::Ready {
                    key: key.clone(),
                    id: *id,
                };
                tracing::info!("authenticating cluster `{key}`");
                if let None = self.driver.get_cluster(*id).await? {
                    self.driver
                        .create_cluster(
                            *id,
                            key.clone(),
                            format!("{key}-{}", util::generate_suffix()),
                        )
                        .await?;
                }
                self.driver
                    .update_cluster(*id, ClusterStatus::Connected, self.addr)
                    .await?;
                let (handle, rx) = ClusterHandle::new(*id, key.clone());
                self.receiver = Some(rx);
                self.registry.register(key, handle);
            }
            (ClusterSessionState::Ready { .. }, ClusterMessage::Ping) => {}
            _ => (),
        };

        Ok(())
    }

    pub async fn close(mut self) -> anyhow::Result<()> {
        match self.state {
            ClusterSessionState::Ready { id, .. } => {
                self.driver
                    .set_cluster_status(id, ClusterStatus::Disconnected)
                    .await?;
            }
            _ => {}
        }
        self.state = ClusterSessionState::Closed;
        Ok(())
    }
}
