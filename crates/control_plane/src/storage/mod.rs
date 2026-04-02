use std::sync::Arc;

use crate::{
    config::Config,
    storage::{driver::Driver, drivers::postrges::PostgresDriver},
};

pub mod driver;
mod drivers;
pub mod model;

pub async fn run(config: Arc<Config>) -> anyhow::Result<impl Driver> {
    let mut driver = PostgresDriver::new(&config.database_url).await?;
    driver.prepare().await?;

    Ok(driver)
}
