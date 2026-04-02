use anyhow::Context;
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::storage::{
    driver::Driver,
    model::cluster::{Cluster, ClusterStatus},
};

pub struct PostgresDriver {
    pool: PgPool,
}

#[async_trait::async_trait]
impl Driver for PostgresDriver {
    async fn create_cluster(&self, key: String) -> anyhow::Result<Cluster> {
        let cluster = sqlx::query_as!(
            Cluster,
            r#"INSERT INTO clusters (key, status) VALUES ($1, $2::cluster_status) RETURNING key, status AS "status: ClusterStatus", last_seen, connected_at, address"#,
            key,
            ClusterStatus::Disconnected as ClusterStatus
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(cluster)
    }

    async fn list_clusters(&self, offset: i64, limit: i64) -> anyhow::Result<Vec<Cluster>> {
        let clusters = sqlx::query_as!(
            Cluster,
            r#"SELECT key, status AS "status: ClusterStatus", last_seen, connected_at, address FROM clusters LIMIT $1 OFFSET $2"#,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(clusters)
    }

    async fn get_cluster(&self, key: String) -> anyhow::Result<Option<Cluster>> {
        let maybe_cluster = sqlx::query_as!(
            Cluster,
            r#"SELECT key, status AS "status: ClusterStatus", last_seen, connected_at, address FROM clusters WHERE key = $1"#,
            key
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(maybe_cluster)
    }

    async fn set_cluster_status(&self, key: String, status: ClusterStatus) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE clusters SET status = $2::cluster_status WHERE key = $1"#,
            key,
            status as ClusterStatus
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn set_cluster_address(
        &self,
        key: String,
        address: std::net::SocketAddr,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE clusters SET address = $2 WHERE key = $1"#,
            key,
            address.to_string()
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update_cluster(
        &self,
        key: String,
        status: ClusterStatus,
        address: std::net::SocketAddr,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE clusters SET status = $2::cluster_status, address = $3 WHERE key = $1"#,
            key,
            status as ClusterStatus,
            address.ip().to_string()
        )
        .execute(&self.pool)
        .await?;
        Ok(())
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
