#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Query {
    pub order_by: String,
    pub signal: Option<String>,
    pub parameter: Option<String>,
}

impl Default for Query {
    fn default() -> Self {
        Self {
            order_by: "ticker".to_string(),
            signal: None,
            parameter: None,
        }
    }
}

impl Query {
    pub fn url(&self, auth: &str) -> String {
        let mut result = format!(
            "https://elite.finviz.com/export?v=111&o={}&auth={}",
            self.order_by, auth
        );

        if let Some(value) = &self.parameter {
            result.push_str(format!("&f={}", value).as_str());
        }

        if let Some(value) = &self.signal {
            result.push_str(format!("&s={}", value).as_str());
        }

        result
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CSVItem {
    #[serde(rename = "No.")]
    pub no: u64,
    #[serde(rename = "Ticker")]
    pub ticker: String,
    #[serde(rename = "Company")]
    pub company: String,
    #[serde(rename = "Sector")]
    pub sector: String,
    #[serde(rename = "Industry")]
    pub industry: String,
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "Market Cap")]
    pub market_cap: Option<f64>,
    #[serde(rename = "P/E")]
    pub pe_ratio: Option<f64>,
    #[serde(rename = "Price")]
    pub price: Option<f64>,
    #[serde(rename = "Change")]
    pub change: Option<String>,
    #[serde(rename = "Volume")]
    pub volume: Option<u64>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct JSONResult {
    pub symbol: Vec<String>,
    pub company: Vec<String>,
    pub sector: Vec<String>,
    pub industry: Vec<String>,
    pub country: Vec<String>,
    pub market_cap: Vec<Option<f64>>,
    pub pe_ratio: Vec<Option<f64>>,
    pub price: Vec<Option<f64>>,
    pub change: Vec<Option<String>>,
    pub volume: Vec<Option<u64>>,
}

pub async fn csv(
    query: &Query,
    auth: &str,
    http_client: &reqwest::Client,
) -> anyhow::Result<Vec<CSVItem>> {
    let url = query.url(auth);
    let response = http_client.get(&url).send().await?;
    let csv_data = response.text().await?;

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_data.as_bytes());

    let mut result: Vec<CSVItem> = Vec::new();

    for record in reader.deserialize() {
        let item: CSVItem = record?;
        result.push(item);
    }

    Ok(result)
}

pub async fn json(csv_data: Vec<CSVItem>) -> anyhow::Result<JSONResult> {
    let mut result = JSONResult::default();

    csv_data.into_iter().for_each(|item| {
        result.symbol.push(item.ticker);
        result.company.push(item.company);
        result.sector.push(item.sector);
        result.industry.push(item.industry);
        result.country.push(item.country);
        result.market_cap.push(item.market_cap);
        result.pe_ratio.push(item.pe_ratio);
        result.price.push(item.price);
        result.change.push(item.change);
        result.volume.push(item.volume);
    });

    Ok(result)
}
