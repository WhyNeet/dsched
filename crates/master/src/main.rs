use std::{process, sync::Arc};

use master::{config::Config, control_plane, identity, node};
use tokio::{signal::unix::SignalKind, task::JoinSet};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    shared::tracing::init();

    tracing::info!("starting master node");

    let id = identity::get().await?;

    let config = Arc::new(Config::from_env().unwrap_or_else(|e| {
        tracing::error!("{}", e);
        process::exit(1);
    }));

    let shutdown = CancellationToken::new();

    let mut set = JoinSet::new();

    set.spawn(control_plane::run(
        Arc::clone(&config),
        id,
        shutdown.clone(),
    ));
    set.spawn(node::run(config, shutdown.clone()));

    let mut sigterm = tokio::signal::unix::signal(SignalKind::terminate()).unwrap();

    loop {
        tokio::select! {
          Some(result) = set.join_next() => match result {
              Ok(_) => tracing::debug!("task exited"),
              Err(e) => tracing::error!("task panicked: {e}"),
          },
          _ = tokio::signal::ctrl_c() => {
            shutdown.cancel();
            tracing::info!("initiating graceful shutdown");
            break;
          }
          _ = sigterm.recv() => {
            shutdown.cancel();
            tracing::info!("initiating graceful shutdown");
            break;
          }
        }
    }

    Ok(())
}
