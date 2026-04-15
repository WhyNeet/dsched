pub mod handler;

use std::{
    collections::HashMap,
    sync::{Arc, atomic::AtomicUsize},
};

use shared::storage::model::job::{Job, JobStatus};
use tokio::sync::Semaphore;
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use uuid::Uuid;

use crate::executor::handler::JobHandler;

pub struct Executor {
    handlers: Arc<HashMap<String, Arc<dyn JobHandler>>>,
    max_tasks: usize,
    running: Arc<AtomicUsize>,
    job_tx: flume::Sender<Job>,
    job_rx: flume::Receiver<Job>,
    job_completion_tx: flume::Sender<(Uuid, JobStatus)>,
    job_completion_rx: flume::Receiver<(Uuid, JobStatus)>,
}

impl Executor {
    pub fn new(max_tasks: usize, handlers: HashMap<String, Arc<dyn JobHandler>>) -> Self {
        let (tx, rx) = flume::unbounded();
        let (comp_tx, comp_rx) = flume::unbounded();

        Self {
            handlers: Arc::new(handlers),
            max_tasks,
            running: Default::default(),
            job_rx: rx,
            job_tx: tx,
            job_completion_rx: comp_rx,
            job_completion_tx: comp_tx,
        }
    }

    pub fn submit_jobs(&self, jobs: Vec<Job>) {
        for job in jobs {
            self.job_tx.send(job).ok();
        }
    }

    pub fn estimate_free_job_slots(&self) -> usize {
        let running = self.running.load(std::sync::atomic::Ordering::Relaxed);
        self.max_tasks.saturating_sub(running)
    }

    pub fn get_completion_rx(&self) -> flume::Receiver<(Uuid, JobStatus)> {
        self.job_completion_rx.clone()
    }

    pub async fn run(self: Arc<Self>, shutdown: CancellationToken) -> anyhow::Result<()> {
        let semaphore = Arc::new(Semaphore::new(self.max_tasks));
        let tasks = TaskTracker::new();

        loop {
            tokio::select! {
              _ = shutdown.cancelled() => {
                break;
              }
              Ok(job) = self.job_rx.recv_async() => {
                let permit = semaphore.clone().acquire_owned().await?;
                let handler = self.handlers.get(&job.r#type).map(Arc::clone);
                let payload = job.payload.0.clone();
                let running = Arc::clone(&self.running);
                let tx = self.job_completion_tx.clone();

                tasks.spawn(async move {
                  let _permit = permit;
                  running.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                  let status = match handler {
                    Some(h) => match h.run(payload).await {
                      Ok(_) => JobStatus::Completed,
                      Err(_) => JobStatus::Failed
                    },
                    None => JobStatus::Failed
                  };
                  running.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                  tx.send((job.id, status)).unwrap();
                });
              }
            }
        }

        tasks.close();
        tasks.wait().await;

        Ok(())
    }
}
