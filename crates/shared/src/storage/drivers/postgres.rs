use crate::storage::{
    driver::Driver,
    model::{
        job::{Job, JobStatus},
        job_definition::JobDefinition,
        node::Node,
    },
};
use anyhow::Context;
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{PgPool, postgres::PgPoolOptions, types::Json};
use uuid::Uuid;

pub struct PostgresDriver {
    pool: PgPool,
}

#[async_trait::async_trait]
impl Driver for PostgresDriver {
    async fn insert_node(&self, node: Node) -> anyhow::Result<()> {
        sqlx::query!(
            "INSERT INTO nodes (id, cluster_key, last_seen) VALUES ($1, $2, $3)",
            node.id,
            node.cluster_key,
            node.last_seen
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    async fn tick_last_seen(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query!("UPDATE nodes SET last_seen = NOW() WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    async fn remove_node(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query!("DELETE FROM nodes WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    async fn count_nodes_by_cluster_key(&self, cluster_key: &str) -> anyhow::Result<i64> {
        let count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM nodes WHERE cluster_key = $1",
            cluster_key
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap();
        Ok(count)
    }
    async fn list_distinct_cluster_keys(&self) -> anyhow::Result<Vec<String>> {
        let keys: Vec<String> = sqlx::query_scalar!("SELECT DISTINCT cluster_key FROM nodes")
            .fetch_all(&self.pool)
            .await?;
        Ok(keys)
    }
    async fn insert_job(&self, job: crate::storage::model::job::Job) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO jobs (id, "type", payload, status, retries, job_definition_id, created_at) VALUES ($1, $2, $3, $4, $5, $6, NOW())"#,
            job.id,
            job.r#type,
            job.payload as Json<Value>,
            job.status as JobStatus,
            0i32,
            job.job_definition_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    async fn get_pending_jobs(
        &self,
        limit: u32,
    ) -> anyhow::Result<Vec<crate::storage::model::job::Job>> {
        let jobs = sqlx::query_as!(
            Job,
            r#"SELECT id, "type", payload, status AS "status: JobStatus", retries, job_definition_id, created_at FROM jobs WHERE status = 'pending' LIMIT $1"#,
            limit as i64
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(jobs)
    }
    async fn create_job_definition(&self, definition: JobDefinition) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO job_definitions (id, type, payload, schedule_type, schedule, max_retries, next_run_at, last_triggered_at, is_enabled, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#,
            definition.id,
            definition.r#type,
            definition.payload as Json<Value>,
            definition.schedule_type,
            definition.schedule as Option<String>,
            definition.max_retries,
            definition.next_run_at,
            definition.last_triggered_at,
            definition.is_enabled,
            definition.created_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    async fn update_job_definition(&self, definition: JobDefinition) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE job_definitions SET type = $2, payload = $3, schedule_type = $4, schedule = $5, max_retries = $6, next_run_at = $7, last_triggered_at = $8, is_enabled = $9 WHERE id = $1"#,
            definition.id,
            definition.r#type,
            definition.payload as Json<Value>,
            definition.schedule_type,
            definition.schedule as Option<String>,
            definition.max_retries,
            definition.next_run_at,
            definition.last_triggered_at,
            definition.is_enabled
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    async fn update_job_definition_schedule(
        &self,
        id: Uuid,
        schedule: Option<String>,
        next_run_at: Option<DateTime<Utc>>,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE job_definitions SET schedule = $2, next_run_at = $3 WHERE id = $1"#,
            id,
            schedule as Option<String>,
            next_run_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    async fn toggle_job_definition_enabled(&self, id: Uuid, enabled: bool) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE job_definitions SET is_enabled = $2 WHERE id = $1"#,
            id,
            enabled
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    async fn delete_job_definition(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query!("DELETE FROM job_definitions WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    async fn get_job_definition(&self, id: Uuid) -> anyhow::Result<Option<JobDefinition>> {
        let definition = sqlx::query_as!(
            JobDefinition,
            r#"SELECT id, "type", payload, schedule_type, schedule, max_retries, next_run_at, last_triggered_at, is_enabled, created_at FROM job_definitions WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(definition)
    }
    async fn list_job_definitions(
        &self,
        limit: u32,
        offset: u32,
    ) -> anyhow::Result<Vec<JobDefinition>> {
        let definitions = sqlx::query_as!(
            JobDefinition,
            r#"SELECT id, "type", payload, schedule_type, schedule, max_retries, next_run_at, last_triggered_at, is_enabled, created_at FROM job_definitions ORDER BY created_at DESC LIMIT $1 OFFSET $2"#,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(definitions)
    }
    async fn get_job_definitions_by_type(
        &self,
        job_type: &str,
    ) -> anyhow::Result<Vec<JobDefinition>> {
        let definitions = sqlx::query_as!(
            JobDefinition,
            r#"SELECT id, "type", payload, schedule_type, schedule, max_retries, next_run_at, last_triggered_at, is_enabled, created_at FROM job_definitions WHERE type = $1 ORDER BY created_at DESC"#,
            job_type
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(definitions)
    }
    async fn get_enabled_job_definitions(&self) -> anyhow::Result<Vec<JobDefinition>> {
        let definitions = sqlx::query_as!(
            JobDefinition,
            r#"SELECT id, "type", payload, schedule_type, schedule, max_retries, next_run_at, last_triggered_at, is_enabled, created_at FROM job_definitions WHERE is_enabled = true ORDER BY created_at DESC"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(definitions)
    }
}

impl PostgresDriver {
    pub async fn new(connection_str: &str) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_str)
            .await?;

        tracing::info!("connected to postrges");

        Ok(Self { pool })
    }

    pub async fn prepare(&mut self) -> anyhow::Result<()> {
        tracing::info!("running migrations");
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .context("failed to run migrations.")
    }
}
