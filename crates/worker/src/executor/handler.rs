#[async_trait::async_trait]
pub trait JobHandler: Send + Sync + 'static {
    async fn run(&self, payload: serde_json::Value) -> anyhow::Result<()>;
}
