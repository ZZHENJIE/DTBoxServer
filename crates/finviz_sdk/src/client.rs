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

    pub async fn screener(
        &self,
        query: &crate::ScreenerQuery,
    ) -> anyhow::Result<crate::screener::JSONResult> {
        let csv_data = crate::screener::csv(query, &self.api_key, &self.http_client).await?;
        crate::screener::json(csv_data).await
    }

    pub async fn quote(
        &self,
        query: &crate::QuoteQuery,
    ) -> anyhow::Result<crate::quote::JSONResult> {
        let csv_data = crate::quote::csv(query, &self.api_key, &self.http_client).await?;
        crate::quote::json(csv_data).await
    }

    pub async fn news(&self, query: &crate::NewsQuery) -> anyhow::Result<crate::news::JSONResult> {
        let csv_data = crate::news::csv(query, &self.api_key, &self.http_client).await?;
        crate::news::json(csv_data).await
    }
}
