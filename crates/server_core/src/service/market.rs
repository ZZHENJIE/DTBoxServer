use crate::config;

#[derive(Debug, Clone)]
pub struct MarketService {
    pub finviz: finviz_sdk::Client,
    pub alpaca_sdk: alpaca_sdk::Client,
}

impl MarketService {
    pub fn new(config: &config::DataSourceConfig) -> Self {
        Self {
            finviz: finviz_sdk::Client::new(&config.finviz_api_key),
            alpaca_sdk: alpaca_sdk::Client::new(&config.alpaca.api_key, &config.alpaca.api_secret),
        }
    }
}
