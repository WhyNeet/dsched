use node_proto::message::NodeMessage;
use std::net::SocketAddr;

#[derive(Debug, Default)]
pub enum NodeSessionState {
    #[default]
    WaitingForAuthentication,
    Ready {
        signature: Vec<u8>,
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

    pub async fn handle_message(&mut self, msg: NodeMessage) -> anyhow::Result<()> {
        match (&self.state, msg) {
            (NodeSessionState::WaitingForAuthentication, NodeMessage::NodeSignature(signature)) => {
                // let key = key.to_string();
                self.state = NodeSessionState::Ready { signature };
                tracing::debug!("authenticating node ({})", self.addr);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub async fn close(mut self) -> anyhow::Result<()> {
        match self.state {
            NodeSessionState::Ready { .. } => {}
            _ => {}
        }
        self.state = NodeSessionState::Closed;
        Ok(())
    }
}
