use cluster_proto::message::ClusterMessage;
use futures_util::SinkExt;
use tokio::net::TcpStream;
use tokio_util::{
    bytes::Bytes,
    codec::{Framed, LengthDelimitedCodec},
    sync::CancellationToken,
};

pub async fn connect(
    addr: &str,
    key: &str,
) -> anyhow::Result<Framed<TcpStream, LengthDelimitedCodec>> {
    let stream: TcpStream = TcpStream::connect(addr).await?;
    let mut framed = Framed::new(stream, LengthDelimitedCodec::new());

    let message =
        rkyv::to_bytes::<rkyv::rancor::Error>(&ClusterMessage::ClusterKey(key.to_string()))?;
    let message = message.to_vec();
    framed.send(Bytes::from(message)).await?;

    Ok(framed)
}

pub async fn maintain_connection<T>(stream: T, shutdown: CancellationToken) {
    _ = stream;

    loop {
        tokio::select! {
          _ = shutdown.cancelled() => {
            tracing::info!("shutting down control plane connection");
            break
          }
        }
    }
    tracing::info!("control plane connection stopped");
}
