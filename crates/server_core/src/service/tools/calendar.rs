#[derive(Clone)]
pub struct CalendarService {
    client: reqwest::Client,
}

impl CalendarService {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
    pub async fn tradingview_economic(
        &self,
        from: chrono::DateTime<chrono::Utc>,
        to: chrono::DateTime<chrono::Utc>,
    ) -> Result<serde_json::Value, anyhow::Error> {
        let from_str = from.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
        let to_str = to.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
        let url = format!(
            "https://economic-calendar.tradingview.com/events?from={}&to={}&countries=US",
            from_str, to_str
        );
        let response = self
            .client
            .get(&url)
            .header("Origin", "https://www.tradingview.com")
            .send()
            .await?;
        if response.status() == reqwest::StatusCode::OK {
            let object: serde_json::Value = response.json().await?;
            let status = object.get("status");
            if let Some(status) = status {
                if status.to_string() != String::from("\"ok\"") {
                    return Err(anyhow::anyhow!("Result error: {}", status.to_string()));
                }
            } else {
                return Err(anyhow::anyhow!("Request error: no status field"));
            }

            if let Some(result) = object.get("result") {
                Ok(result.clone())
            } else {
                Err(anyhow::anyhow!("Result error: no result field"))
            }
        } else {
            Err(anyhow::anyhow!(
                "Request error: status code {}",
                response.status()
            ))
        }
    }
}
