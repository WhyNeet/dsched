#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub control_plane_url: String,
    #[serde(default = "default_tcp_port")]
    pub tcp_port: u16,
    pub cluster_key: String,
}

fn default_tcp_port() -> u16 {
    8081
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(config::Config::builder()
            .add_source(config::File::with_name("config").required(false))
            .add_source(config::File::with_name("config.local").required(false))
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_"),
            )
            .build()?
            .try_deserialize()?)
    }
}
