use std::net::SocketAddr;

use anyhow::Context;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, postgres::PgPoolOptions};
use uuid::Uuid;

use crate::storage::{
    driver::Driver,
    model::cluster::{Cluster, ClusterStatus},
};

pub struct PostgresDriver {
    pool: PgPool,
}

#[async_trait::async_trait]
impl Driver for PostgresDriver {
    async fn create_cluster(
        &self,
        id: Uuid,
        key: String,
        display_name: String,
    ) -> anyhow::Result<Cluster> {
        let cluster = sqlx::query_as!(
            Cluster,
            r#"INSERT INTO clusters (id, key, display_name, status) VALUES ($1, $2, $3, $4::cluster_status) RETURNING id, key, display_name, status AS "status: ClusterStatus", last_seen, connected_at, address"#,
            id,
            key,
            display_name,
            ClusterStatus::Disconnected as ClusterStatus
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(cluster)
    }

    async fn get_cluster(&self, id: Uuid) -> anyhow::Result<Option<Cluster>> {
        let cluster = sqlx::query_as!(
            Cluster,
            r#"SELECT id, key, display_name, status AS "status: ClusterStatus", last_seen, connected_at, address FROM clusters WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(cluster)
    }

    async fn list_clusters(&self, offset: i64, limit: i64) -> anyhow::Result<Vec<Cluster>> {
        let clusters = sqlx::query_as!(
            Cluster,
            r#"SELECT id, key, display_name, status AS "status: ClusterStatus", last_seen, connected_at, address FROM clusters LIMIT $1 OFFSET $2"#,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(clusters)
    }

    async fn get_clusters_by_key(&self, key: String) -> anyhow::Result<Vec<Cluster>> {
        let clusters = sqlx::query_as!(
            Cluster,
            r#"SELECT id, key, display_name, status AS "status: ClusterStatus", last_seen, connected_at, address FROM clusters WHERE key = $1"#,
            key
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(clusters)
    }

    async fn set_cluster_status(&self, id: Uuid, status: ClusterStatus) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE clusters SET status = $2::cluster_status WHERE id = $1"#,
            id,
            status as ClusterStatus
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn set_cluster_address(
        &self,
        id: Uuid,
        address: std::net::SocketAddr,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE clusters SET address = $2 WHERE id = $1"#,
            id,
            address.to_string()
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update_cluster(
        &self,
        id: Uuid,
        status: ClusterStatus,
        address: std::net::SocketAddr,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE clusters SET status = $2::cluster_status, address = $3 WHERE id = $1"#,
            id,
            status as ClusterStatus,
            address.ip().to_string()
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn set_cluster_status_by_key(
        &self,
        key: String,
        status: ClusterStatus,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE clusters SET status = $2::cluster_status WHERE key = $1"#,
            key,
            status as ClusterStatus
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn set_cluster_address_by_key(
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

    async fn update_cluster_by_key(
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

    async fn upsert_cluster(
        &self,
        id: Uuid,
        key: String,
        display_name: String,
        status: ClusterStatus,
        address: SocketAddr,
        last_seen: DateTime<Utc>,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO clusters (id, key, display_name, status, address, last_seen) VALUES ($1, $2, $3, $4::cluster_status, $5, $6)
        ON CONFLICT (id) DO UPDATE SET display_name = $3, status = $4::cluster_status, address = $5, last_seen = $6"#,
            id,
            key,
            display_name,
            status as ClusterStatus,
            address.ip().to_string(),
            last_seen
        ).fetch_one(&self.pool).await?;
        Ok(())
    }

    async fn count_clusters_by_key(&self, key: String) -> anyhow::Result<i64> {
        Ok(
            sqlx::query_scalar!(r#"SELECT COUNT(*) FROM clusters WHERE key = $1"#, key)
                .fetch_one(&self.pool)
                .await?
                .unwrap(),
        )
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
