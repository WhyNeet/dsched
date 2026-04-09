use std::sync::Arc;

use shared::storage::driver::Driver;

use crate::config::Config;

pub async fn run(config: Arc<Config>) -> anyhow::Result<impl Driver> {
    shared::storage::run(config.database_url.clone(), true).await
}
