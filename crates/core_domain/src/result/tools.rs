use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingviewEconomicCalendarItem {
    pub actual: Option<f64>,
    pub actual_raw: Option<f64>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub comment: Option<String>,
    pub country: String,
    pub currency: String,
    pub date: DateTime<Utc>,
    pub forecast: Option<f64>,
    pub forecast_raw: Option<f64>,
    pub id: String,
    pub importance: i8,
    pub indicator: String,
    pub period: String,
    pub previous: Option<f64>,
    pub previous_raw: Option<f64>,
    #[serde(default)]
    pub reference_date: Option<DateTime<Utc>>,
    #[serde(default)]
    pub scale: Option<String>,
    pub source: String,
    #[serde(rename = "source_url")]
    pub source_url: String,
    #[serde(default)]
    pub ticker: Option<String>,
    pub title: String,
    #[serde(default)]
    pub unit: Option<String>,
}
