use std::time::Duration;

use cluster_proto::message::ClusterMessage;
use futures_util::SinkExt;
use tokio::{net::TcpStream, time};
use tokio_util::{
    bytes::Bytes,
    codec::{Framed, LengthDelimitedCodec},
    sync::CancellationToken,
};
use uuid::Uuid;

pub async fn connect(
    addr: &str,
    id: Uuid,
    key: &str,
) -> anyhow::Result<Framed<TcpStream, LengthDelimitedCodec>> {
    let stream: TcpStream = TcpStream::connect(addr).await?;
    let mut framed = Framed::new(stream, LengthDelimitedCodec::new());

    let message =
        rkyv::to_bytes::<rkyv::rancor::Error>(&ClusterMessage::ClusterAuth(id, key.to_string()))?;
    let message = message.to_vec();
    framed.send(Bytes::from(message)).await?;

    Ok(framed)
}

pub async fn maintain_connection(
    mut stream: Framed<TcpStream, LengthDelimitedCodec>,
    shutdown: CancellationToken,
) {
    let mut interval = time::interval(Duration::from_secs(5));

    loop {
        tokio::select! {
          _ = interval.tick() => {
            let message =
                rkyv::to_bytes::<rkyv::rancor::Error>(&ClusterMessage::Ping).unwrap();
            let message = message.to_vec();
            match stream.send(Bytes::from(message)).await {
              Ok(_) => {

              }
              Err(e) => tracing::debug!("failed to ping control plane: {e}"),
            }
          }
          _ = shutdown.cancelled() => {
            tracing::info!("shutting down control plane connection");
            break
          }
        }
    }
    tracing::info!("control plane connection stopped");
}
