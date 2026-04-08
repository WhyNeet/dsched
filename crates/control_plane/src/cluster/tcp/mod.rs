use std::{net::SocketAddr, sync::Arc};

use cluster_proto::message::ClusterMessage;
use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_util::{
    bytes::Bytes,
    codec::{Framed, LengthDelimitedCodec},
    sync::CancellationToken,
};

use crate::{
    cluster::{registry::ClusterRegistry, tcp::session::ClusterSession},
    config::Config,
    storage::driver::Driver,
};

mod session;

pub async fn run(
    config: Arc<Config>,
    driver: Arc<dyn Driver>,
    registry: ClusterRegistry,
    shutdown: CancellationToken,
) -> anyhow::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", config.tcp_port)).await?;

    tracing::info!("listening on port {}", config.tcp_port);

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(
            stream,
            addr,
            Arc::clone(&driver),
            registry.clone(),
            shutdown.clone(),
        ));
    }

    Ok(())
}

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    driver: Arc<dyn Driver>,
    registry: ClusterRegistry,
    shutdown: CancellationToken,
) {
    let mut bytes = Framed::new(stream, LengthDelimitedCodec::new());

    let mut session = ClusterSession::new(driver, addr, registry);

    tracing::debug!("created new session");

    loop {
        tokio::select! {
          _ = shutdown.cancelled() => {
            break
          }
          result = bytes.next() => match result {
            Some(Ok(msg)) => {
              match rkyv::from_bytes::<ClusterMessage, rkyv::rancor::Error>(&msg) {
                Ok(msg) => match session.handle_message(&msg).await {
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
              break
            }
          },
          Ok(msg) = async {
            match &session.receiver {
              Some(rx) => rx.recv_async().await,
              None => std::future::pending().await
            }
          } => {
            let message = rkyv::to_bytes::<rkyv::rancor::Error>(&msg).unwrap();
            let message = message.to_vec();
            match bytes.send(Bytes::from(message)).await {
              Err(e) => tracing::debug!("failed to send message to cluster: {e}"),
              _ => ()
            }
          }
        }
    }

    match session.close().await {
        Ok(_) => {}
        Err(e) => tracing::error!("failed to close session: {e}"),
    }

    tracing::debug!("closed session");
}
