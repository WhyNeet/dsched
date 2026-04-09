#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub cluster_key: String,
    pub max_tasks: usize,
}
