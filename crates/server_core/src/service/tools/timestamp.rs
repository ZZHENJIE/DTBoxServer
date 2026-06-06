#[derive(Clone)]
pub struct TimestampService {
    client: reqwest::Client,
}

impl TimestampService {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
    pub async fn akamai(&self) -> Result<u64, anyhow::Error> {
        let response = self.client.get("https://time.akamai.com").send().await?;
        let text = response.text().await?;
        let timestamp = text.parse::<u64>()?;
        Ok(timestamp)
    }
}
