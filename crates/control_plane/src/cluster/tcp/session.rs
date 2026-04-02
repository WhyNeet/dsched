use std::{net::SocketAddr, sync::Arc};

use cluster_proto::message::ClusterMessage;

use crate::storage::driver::Driver;

#[derive(Debug, Default)]
pub enum ClusterSessionState {
    #[default]
    WaitingForAuthentication,
    Ready {
        key: String,
    },
    Closed,
}

pub struct ClusterSession {
    state: ClusterSessionState,
    driver: Arc<dyn Driver>,
    addr: SocketAddr,
}

impl ClusterSession {
    pub fn new(driver: Arc<dyn Driver>, addr: SocketAddr) -> Self {
        Self {
            state: Default::default(),
            driver,
            addr,
        }
    }

    pub async fn handle_message(&mut self, msg: &ClusterMessage) -> anyhow::Result<()> {
        match (&self.state, msg) {
            (ClusterSessionState::WaitingForAuthentication, ClusterMessage::ClusterKey(key)) => {
                let key = key.to_string();
                self.state = ClusterSessionState::Ready { key: key.clone() };
                tracing::info!("authenticating cluster `{key}`");
                self.driver
                    .update_cluster(
                        key,
                        crate::storage::model::cluster::ClusterStatus::Connected,
                        self.addr,
                    )
                    .await
            }
            _ => Ok(()),
        }
    }

    pub async fn close(mut self) -> anyhow::Result<()> {
        match self.state {
            ClusterSessionState::Ready { key } => {
                self.driver
                    .set_cluster_status(
                        key,
                        crate::storage::model::cluster::ClusterStatus::Disconnected,
                    )
                    .await?;
            }
            _ => {}
        }
        self.state = ClusterSessionState::Closed;
        Ok(())
    }
}
