use chrono::Utc;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TradingviewEconomicCalendarPayload {
    pub from: chrono::DateTime<Utc>,
    pub to: chrono::DateTime<Utc>,
}
