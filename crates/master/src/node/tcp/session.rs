use std::net::SocketAddr;

#[derive(Debug, Default)]
pub enum NodeSessionState {
    #[default]
    WaitingForAuthentication,
    Ready {
        key: String,
    },
    Closed,
}

pub struct NodeSession {
    state: NodeSessionState,
    addr: SocketAddr,
}

impl NodeSession {
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            state: Default::default(),
            addr,
        }
    }

    pub async fn handle_message(&mut self, msg: &NodeMessage) -> anyhow::Result<()> {
        match (&self.state, msg) {
            (NodeSessionState::WaitingForAuthentication, NodeMessage::ClusterKey(key)) => {
                let key = key.to_string();
                self.state = NodeSessionState::Ready { key: key.clone() };
                tracing::info!("authenticating cluster `{key}`");
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub async fn close(mut self) -> anyhow::Result<()> {
        match self.state {
            NodeSessionState::Ready { key } => {}
            _ => {}
        }
        self.state = NodeSessionState::Closed;
        Ok(())
    }
}
