use std::{net::SocketAddr, sync::Arc};

use futures_util::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{Framed, LinesCodec};

use crate::{
    cluster::tcp::{message::ArchivedClusterMessage, session::ClusterSession},
    config::Config,
    storage::driver::Driver,
};

mod message;
mod session;

pub async fn run(config: Arc<Config>, driver: Arc<dyn Driver>) -> anyhow::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", config.tcp_port)).await?;

    tracing::info!("TCP listening on port {}", config.tcp_port);

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr, Arc::clone(&driver)));
    }

    Ok(())
}

async fn handle_connection(stream: TcpStream, addr: SocketAddr, driver: Arc<dyn Driver>) {
    let mut lines = Framed::new(stream, LinesCodec::new());

    let mut session = ClusterSession::new(driver, addr);

    tracing::debug!("created new session");

    loop {
        tokio::select! {
          result = lines.next() => match result {
            Some(Ok(msg)) => {
              match rkyv::access::<ArchivedClusterMessage, rkyv::rancor::Error>(msg.as_bytes()) {
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
              match session.close().await {
                Ok(_) => {},
                Err(e) => tracing::error!("failed to close session: {e}")
                }
              break
            }
          }
        }
    }

    tracing::debug!("closed session");
}
