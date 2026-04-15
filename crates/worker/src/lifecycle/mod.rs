use chrono::Utc;
use shared::storage::{driver::Driver, model::node::Node};
use std::{sync::Arc, time::Duration};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::{config::Config, executor::Executor};

pub async fn run(
    driver: Arc<dyn Driver>,
    config: Arc<Config>,
    executor: Arc<Executor>,
    id: Uuid,
    shutdown: CancellationToken,
) -> anyhow::Result<()> {
    let node = Node {
        id,
        cluster_key: config.cluster_key.clone(),
        last_seen: Utc::now(),
    };
    tracing::info!("starting lifecycle");
    driver.insert_node(node).await?;

    let mut interval = tokio::time::interval(Duration::from_secs(10));

    loop {
        tokio::select! {
          _ = shutdown.cancelled() => {
            break;
          }
          _ = interval.tick() => {
            driver.tick_last_seen(id).await?;
            let jobs = driver.get_pending_jobs(executor.estimate_free_job_slots() as u32).await?;
            executor.submit_jobs(jobs);
          }
        }
    }

    tracing::info!("shutting down");

    driver.remove_node(id).await?;

    Ok(())
}
