use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use crate::config::Config;

// mod tcp;

pub async fn run(config: Arc<Config>, shutdown: CancellationToken) -> anyhow::Result<()> {
    Ok(())
}
