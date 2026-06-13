use reqwest::header::{HeaderMap, HeaderValue};

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

    pub fn header(&self) -> anyhow::Result<HeaderMap> {
        let mut map = HeaderMap::new();
        map.insert(
            "APCA-API-KEY-ID",
            HeaderValue::from_str(self.api_key.as_str())?,
        );
        map.insert(
            "APCA-API-SECRET-KEY",
            HeaderValue::from_str(&self.api_secret.as_str())?,
        );
        map.insert("accept", HeaderValue::from_static("application/json"));
        Ok(map)
    }

    pub async fn snapshot(
        &self,
        query: &crate::SnapshotQuery,
    ) -> anyhow::Result<crate::snapshot::Response> {
        crate::snapshot::fetch(self.http_client.get(query.url()).headers(self.header()?)).await
    }
}
