use std::{process, sync::Arc};

use master::{config::Config, control_plane};
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    shared::tracing::init();

    tracing::info!("starting control plane");

    let config = Arc::new(Config::from_env().unwrap_or_else(|e| {
        tracing::error!("{}", e);
        process::exit(1);
    }));

    let shutdown = CancellationToken::new();

    let mut set = JoinSet::new();

    set.spawn(control_plane::run(config, shutdown.clone()));

    loop {
        tokio::select! {
          Some(result) = set.join_next() => match result {
              Ok(_) => tracing::debug!("task exited"),
              Err(e) => tracing::error!("task panicked: {e}"),
          },
          _ = tokio::signal::ctrl_c() => {
            shutdown.cancel();
            break;
          }
        }
    }

    Ok(())
}
