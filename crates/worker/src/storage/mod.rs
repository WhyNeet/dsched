use std::sync::Arc;

use shared::storage::{self, driver::Driver};

use crate::config::Config;

pub async fn run(config: Arc<Config>) -> anyhow::Result<impl Driver> {
    storage::run(config.database_url.clone(), false).await
}
