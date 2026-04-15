use uuid::Uuid;

use crate::storage::model::{
    job::{Job, JobStatus},
    job_definition::JobDefinition,
    node::Node,
};

#[async_trait::async_trait]
pub trait Driver: Send + Sync {
    async fn insert_node(&self, node: Node) -> anyhow::Result<()>;
    async fn remove_node(&self, id: Uuid) -> anyhow::Result<()>;
    async fn tick_last_seen(&self, id: Uuid) -> anyhow::Result<()>;
    async fn count_nodes_by_cluster_key(&self, cluster_key: &str) -> anyhow::Result<i64>;
    async fn list_distinct_cluster_keys(&self) -> anyhow::Result<Vec<String>>;

    async fn create_job_definition(&self, definition: JobDefinition) -> anyhow::Result<()>;
    async fn update_job_definition(&self, definition: JobDefinition) -> anyhow::Result<()>;
    async fn get_job_definition(&self, id: Uuid) -> anyhow::Result<Option<JobDefinition>>;
    async fn list_job_definitions(
        &self,
        limit: u32,
        offset: u32,
    ) -> anyhow::Result<Vec<JobDefinition>>;
    async fn delete_job_definition(&self, id: Uuid) -> anyhow::Result<()>;
    async fn get_job_definitions_by_type(
        &self,
        job_type: &str,
    ) -> anyhow::Result<Vec<JobDefinition>>;
    async fn get_enabled_job_definitions(&self) -> anyhow::Result<Vec<JobDefinition>>;
    async fn update_job_definition_next_run_at(
        &self,
        id: Uuid,
        next_run_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> anyhow::Result<()>;
    async fn update_job_definition_schedule(
        &self,
        id: Uuid,
        schedule_type: String,
        schedule: Option<String>,
        next_run_at: chrono::DateTime<chrono::Utc>,
    ) -> anyhow::Result<()>;
    async fn toggle_job_definition_enabled(&self, id: Uuid, enabled: bool) -> anyhow::Result<()>;
    async fn get_unscheduled_job_definitions(
        &self,
        limit: u32,
    ) -> anyhow::Result<Vec<JobDefinition>>;

    async fn insert_job(&self, job: Job) -> anyhow::Result<()>;
    async fn update_job_status(&self, id: Uuid, status: JobStatus) -> anyhow::Result<()>;
    async fn get_pending_jobs(&self, batch_size: u32) -> anyhow::Result<Vec<Job>>;
}
