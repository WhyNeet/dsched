use std::{process, sync::Arc};

use control_plane::{config::Config, http, storage};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    control_plane::tracing::init();

    tracing::info!("starting control plane");

    let config = Arc::new(Config::from_env().unwrap_or_else(|e| {
        tracing::error!("{}", e);
        process::exit(1);
    }));

    let driver = storage::run(Arc::clone(&config)).await.unwrap_or_else(|e| {
        tracing::error!("{}", e);
        process::exit(1);
    });

    let mut set = JoinSet::new();

    set.spawn(http::run(Arc::clone(&config), driver));

    while let Some(result) = set.join_next().await {
        match result {
            Ok(_) => tracing::debug!("task exited"),
            Err(e) => tracing::error!("task panicked: {e}"),
        }
    }

    Ok(())
}
