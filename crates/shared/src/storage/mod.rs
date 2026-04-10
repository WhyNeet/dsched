use crate::storage::{driver::Driver, drivers::postgres::PostgresDriver};

pub mod driver;
mod drivers;
pub mod model;

pub async fn run(database_url: String, should_migrate: bool) -> anyhow::Result<impl Driver> {
    let mut driver = PostgresDriver::new(&database_url).await?;
    if should_migrate {
        driver.prepare().await?;
    }

    Ok(driver)
}
