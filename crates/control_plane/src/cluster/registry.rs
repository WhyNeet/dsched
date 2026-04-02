use std::sync::Arc;

use dashmap::DashMap;

use crate::cluster::ClusterHandle;

#[derive(Clone, Default)]
pub struct ClusterRegistry {
    clusters: Arc<DashMap<String, ClusterHandle>>,
}

impl ClusterRegistry {
    pub fn get(&self, key: &str) -> Option<ClusterHandle> {
        self.clusters.get(key).map(|c| c.clone())
    }

    pub fn register(&self, key: String, handle: ClusterHandle) {
        self.clusters.insert(key, handle);
    }

    pub fn alive_clusters(&self) -> Vec<String> {
        self.clusters.iter().map(|r| r.key.clone()).collect()
    }
}
