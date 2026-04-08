use std::{process, sync::Arc};

use control_plane::{
    cluster::{self, registry::ClusterRegistry},
    config::Config,
    http,
    storage::{self, driver::Driver},
};
use tokio::{signal::unix::SignalKind, task::JoinSet};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    shared::tracing::init();

    tracing::info!("starting control plane");

    let config = Arc::new(Config::from_env().unwrap_or_else(|e| {
        tracing::error!("{}", e);
        process::exit(1);
    }));

    let driver = storage::run(Arc::clone(&config)).await.unwrap_or_else(|e| {
        tracing::error!("{}", e);
        process::exit(1);
    });

    let driver: Arc<dyn Driver> = Arc::new(driver);
    let registry = ClusterRegistry::default();

    let shutdown = CancellationToken::new();

    let mut set = JoinSet::new();

    set.spawn(http::run(
        Arc::clone(&config),
        Arc::clone(&driver),
        shutdown.clone(),
    ));
    set.spawn(cluster::tcp::run(Arc::clone(&config), driver, registry));

    let mut sigterm = tokio::signal::unix::signal(SignalKind::terminate()).unwrap();

    loop {
        tokio::select! {
          Some(result) = set.join_next() => {
              match result {
                  Ok(_) => tracing::debug!("task exited"),
                  Err(e) => tracing::error!("task panicked: {e}"),
              }
          }
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
