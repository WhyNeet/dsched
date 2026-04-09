use anyhow::Context;
use serde_json::Value;
use sqlx::{PgPool, postgres::PgPoolOptions, types::Json};
use uuid::Uuid;

use crate::storage::{
    driver::Driver,
    model::{
        job::{Job, JobStatus},
        node::Node,
    },
};

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

    async fn insert_job(&self, job: crate::storage::model::job::Job) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO jobs (id, "type", payload, run_at, status) VALUES ($1, $2, $3, $4, $5)"#,
            job.id,
            job.r#type,
            job.payload as Json<Value>,
            job.run_at,
            job.status as JobStatus
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
            r#"SELECT id, "type", payload, run_at, status AS "status: JobStatus" FROM jobs WHERE status = 'pending' AND run_at >= NOW() LIMIT $1"#,
            limit as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(jobs)
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
