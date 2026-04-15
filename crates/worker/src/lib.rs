pub mod config;
mod executor;
mod lifecycle;
mod storage;

pub use executor::handler::JobHandler;
use uuid::Uuid;

use std::{collections::HashMap, sync::Arc};

use config::Config;
use shared::storage::driver::Driver;
use tokio::{signal::unix::SignalKind, task::JoinSet};
use tokio_util::sync::CancellationToken;

use crate::executor::Executor;

pub struct Worker {
    config: Arc<Config>,
    handlers: HashMap<String, Arc<dyn JobHandler>>,
}

impl Worker {
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
            handlers: Default::default(),
        }
    }

    pub fn register(&mut self, r#type: String, handler: impl JobHandler) {
        self.handlers.insert(r#type, Arc::new(handler));
    }

    pub async fn run(self) -> anyhow::Result<()> {
        tracing::info!("starting worker node");

        let node_id = Uuid::new_v4();

        let driver = storage::run(Arc::clone(&self.config)).await?;
        let driver: Arc<dyn Driver> = Arc::new(driver);

        let executor = Arc::new(Executor::new(self.config.max_tasks, self.handlers));

        let shutdown = CancellationToken::new();

        let mut set = JoinSet::new();

        set.spawn(lifecycle::run(
            driver,
            Arc::clone(&self.config),
            Arc::clone(&executor),
            node_id,
            shutdown.clone(),
        ));

        set.spawn(executor.run(shutdown.clone()));

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
}
