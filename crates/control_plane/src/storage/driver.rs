use std::net::SocketAddr;

use crate::storage::model::cluster::{Cluster, ClusterStatus};

#[async_trait::async_trait]
pub trait Driver: Send + Sync {
    async fn create_cluster(&self, key: String) -> anyhow::Result<Cluster>;
    async fn list_clusters(&self, offset: i64, limit: i64) -> anyhow::Result<Vec<Cluster>>;
    async fn get_cluster(&self, key: String) -> anyhow::Result<Option<Cluster>>;
    async fn set_cluster_status(&self, key: String, status: ClusterStatus) -> anyhow::Result<()>;
    async fn set_cluster_address(&self, key: String, address: SocketAddr) -> anyhow::Result<()>;
    async fn update_cluster(
        &self,
        key: String,
        status: ClusterStatus,
        address: SocketAddr,
    ) -> anyhow::Result<()>;
}
