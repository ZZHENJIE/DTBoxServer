#[derive(Debug, Clone)]
pub struct Client {
    api_key: String,
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            http_client: reqwest::Client::new(),
        }
    }
}
