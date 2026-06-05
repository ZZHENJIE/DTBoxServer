#[derive(Debug, Clone)]
pub struct Client {
    api_key: String,
    api_secret: String,
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(api_key: &str, api_secret: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
            http_client: reqwest::Client::new(),
        }
    }
}
