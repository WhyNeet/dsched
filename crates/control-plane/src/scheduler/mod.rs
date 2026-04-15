use std::{str::FromStr, sync::Arc, time::Duration};

use chrono::Utc;
use croner::Cron;
use shared::storage::{
    driver::Driver,
    model::job::{Job, JobStatus},
};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::config::Config;

pub async fn run(
    config: Arc<Config>,
    driver: Arc<dyn Driver>,
    shutdown: CancellationToken,
) -> anyhow::Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    let mut reaper_interval =
        tokio::time::interval(Duration::from_secs(config.reaper_interval_secs));

    loop {
        tokio::select! {
          _ = interval.tick() => {
            let jobs = driver.get_unscheduled_job_definitions(100).await.unwrap();

            futures_util::future::join_all(jobs.into_iter().map(async |job| {
              if let Some(ref schedule) = job.schedule {
                driver.update_job_definition_next_run_at(job.id, Some(Cron::from_str(schedule)?.find_next_occurrence(&Utc::now(), true)?)).await?;
              } else {
                driver.toggle_job_definition_enabled(job.id, false).await?;
              }

                driver.insert_job(Job {
                    id: Uuid::new_v4(),
                    payload: job.payload,
                    job_definition_id: Some(job.id),
                    retries: job.max_retries,
                    status: JobStatus::Pending,
                    r#type: job.r#type,
                    created_at: Utc::now(),
                }).await
            }))
            .await;
          },
          _ = reaper_interval.tick() => {
            tracing::debug!("running reaper");
          },
          _ = shutdown.cancelled() => {
            break;
          }
        }
    }

    Ok(())
}
