use tracing_subscriber::{filter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            filter::EnvFilter::builder()
                .with_default_directive(filter::LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();
}
