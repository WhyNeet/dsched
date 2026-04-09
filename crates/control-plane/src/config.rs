#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub database_url: String,
    #[serde(default = "default_http_port")]
    pub http_port: u16,
    #[serde(default = "default_reaper_interval")]
    pub reaper_interval_secs: u64,
}

fn default_http_port() -> u16 {
    8080
}

fn default_reaper_interval() -> u64 {
    60
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(config::Config::builder()
            .add_source(config::File::with_name("config").required(false))
            .add_source(config::File::with_name("config.local").required(false))
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .convert_case(config::Case::Snake),
            )
            .build()?
            .try_deserialize()?)
    }
}
