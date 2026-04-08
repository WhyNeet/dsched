use std::{net::SocketAddr, sync::Arc};

use futures_util::{SinkExt, StreamExt};
use node_proto::message::NodeMessage;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::{
    codec::{Framed, LengthDelimitedCodec},
    sync::CancellationToken,
    task::TaskTracker,
};

use crate::{config::Config, node::tcp::session::NodeSession};

mod session;

pub async fn run(config: Arc<Config>, shutdown: CancellationToken) -> anyhow::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", config.tcp_port)).await?;
    let tracker = TaskTracker::new();

    tracing::info!("listening on port {}", config.tcp_port);

    loop {
        tokio::select! {
          _ = shutdown.cancelled() => {
            break;
          }
          result = listener.accept() => {
            if let Ok((stream, addr)) = result {
              tracker.spawn(handle_connection(stream, addr, shutdown.clone()));
            }
          }
        }
    }

    tracing::info!("Waiting for all connections to finish work");

    tracker.close();
    tracker.wait().await;

    Ok(())
}

async fn handle_connection(stream: TcpStream, addr: SocketAddr, shutdown: CancellationToken) {
    let mut bytes = Framed::new(stream, LengthDelimitedCodec::new());

    let mut session = NodeSession::new(addr);

    tracing::debug!("created new session");

    loop {
        tokio::select! {
          _ = shutdown.cancelled() => {
            break;
          }
          result = bytes.next() => match result {
            Some(Ok(msg)) => {
              match rkyv::from_bytes::<NodeMessage, rkyv::rancor::Error>(&msg) {
                Ok(msg) => match session.handle_message(msg).await {
                  Ok(_) => {},
                  Err(e) => tracing::error!("error handling message: {e}")
                },
                Err(e) => tracing::error!("failed to deserialize cluster message: {e}")
              }
            },
            Some(Err(e)) => {
              tracing::error!("{e}");
            },
            None => {
              break;
            }
          }
        }
    }

    match session.close().await {
        Ok(_) => {}
        Err(e) => tracing::error!("failed to close session: {e}"),
    };
    bytes.close().await.ok();

    tracing::debug!("closed session");
}
