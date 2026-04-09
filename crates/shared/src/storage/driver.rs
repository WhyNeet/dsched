use uuid::Uuid;

use crate::storage::model::{job::Job, node::Node};

#[async_trait::async_trait]
pub trait Driver: Send + Sync {
    async fn insert_node(&self, node: Node) -> anyhow::Result<()>;
    async fn remove_node(&self, id: Uuid) -> anyhow::Result<()>;
    async fn tick_last_seen(&self, id: Uuid) -> anyhow::Result<()>;

    async fn insert_job(&self, job: Job) -> anyhow::Result<()>;
    async fn get_pending_jobs(&self, batch_size: u32) -> anyhow::Result<Vec<Job>>;
}
