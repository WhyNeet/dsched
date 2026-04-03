use std::net::SocketAddr;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::storage::model::cluster::{Cluster, ClusterStatus};

#[async_trait::async_trait]
pub trait Driver: Send + Sync {
    async fn create_cluster(
        &self,
        id: Uuid,
        key: String,
        display_name: String,
    ) -> anyhow::Result<Cluster>;
    async fn list_clusters(&self, offset: i64, limit: i64) -> anyhow::Result<Vec<Cluster>>;
    async fn get_cluster(&self, id: Uuid) -> anyhow::Result<Option<Cluster>>;
    async fn get_clusters_by_key(&self, key: String) -> anyhow::Result<Vec<Cluster>>;
    async fn set_cluster_status_by_key(
        &self,
        key: String,
        status: ClusterStatus,
    ) -> anyhow::Result<()>;
    async fn set_cluster_address_by_key(
        &self,
        key: String,
        address: SocketAddr,
    ) -> anyhow::Result<()>;
    async fn update_cluster_by_key(
        &self,
        key: String,
        status: ClusterStatus,
        address: SocketAddr,
    ) -> anyhow::Result<()>;

    async fn set_cluster_status(&self, id: Uuid, status: ClusterStatus) -> anyhow::Result<()>;
    async fn set_cluster_address(&self, id: Uuid, address: SocketAddr) -> anyhow::Result<()>;
    async fn update_cluster(
        &self,
        id: Uuid,
        status: ClusterStatus,
        address: SocketAddr,
    ) -> anyhow::Result<()>;

    async fn upsert_cluster(
        &self,
        id: Uuid,
        key: String,
        display_name: String,
        status: ClusterStatus,
        address: SocketAddr,
        last_seen: DateTime<Utc>,
    ) -> anyhow::Result<()>;

    async fn count_clusters_by_key(&self, key: String) -> anyhow::Result<i64>;
}
