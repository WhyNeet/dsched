use anyhow::Context;
use tracing_subscriber::{filter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use worker::{JobHandler, Worker, config::Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            filter::EnvFilter::builder()
                .with_default_directive(filter::LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let mut worker = Worker::new(Config {
        cluster_key: "example".to_string(),
        database_url: std::env::var("DATABASE_URL")?,
        max_tasks: 10,
    });

    worker.register("send_email".to_string(), EmailJobHandler);

    worker.run().await
}

struct EmailJobHandler;

#[async_trait::async_trait]
impl JobHandler for EmailJobHandler {
    async fn run(&self, payload: serde_json::Value) -> anyhow::Result<()> {
        let payload = payload.as_object().context("payload is not an object")?;
        tracing::info!(
            "sending email to: {}",
            payload
                .get("address")
                .context("missing email address in payload")?
        );
        Ok(())
    }
}
