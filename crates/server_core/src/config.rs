use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database_url: String,
    pub jwt: JWTConfig,
    pub logging: LoggingConfig,
    pub data_source: DataSourceConfig,
    pub rate_limiter: RateLimiterConfig,
}

#[derive(Debug, Deserialize)]
pub struct RateLimiterConfig {
    pub max_requests: u64,
    pub window_seconds: u64,
}

#[derive(Debug, Deserialize)]
pub struct JWTConfig {
    pub secret: String,
    pub access_token_expire_minutes: u64,
    pub refresh_token_expire_days: u64,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub web_dir: String,
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub log_dir: String,
}

#[derive(Debug, Deserialize)]
pub struct DataSourceConfig {
    pub finviz_api_key: String,
    pub alpaca: AlpacaConfig,
}

#[derive(Debug, Deserialize)]
pub struct AlpacaConfig {
    pub api_key: String,
    pub api_secret: String,
}

impl Config {
    pub fn load_file(path: &str) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }
}
