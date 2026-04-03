use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::config::Config;

mod tcp;

pub async fn run(config: Arc<Config>, id: Uuid, shutdown: CancellationToken) -> anyhow::Result<()> {
    tracing::info!("connecting to control plane");
    let stream = tcp::connect(&config.control_plane_url, id, &config.cluster_key).await?;

    tracing::info!("cluster authorization successful");

    tokio::spawn(tcp::maintain_connection(stream, shutdown));

    Ok(())
}
