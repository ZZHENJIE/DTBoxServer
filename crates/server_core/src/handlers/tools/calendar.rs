use std::sync::Arc;

use axum::{Json, extract::State};
use core_domain::payload;

use crate::{error::ErrorCode, response::APIResponse, state::AppState};

pub async fn tradingview_economic(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<payload::tools::TradingviewEconomicCalendarPayload>,
) -> APIResponse<serde_json::Value> {
    match state
        .service()
        .tools
        .calendar
        .tradingview_economic(payload.from, payload.to)
        .await
    {
        Ok(timestamp) => APIResponse::success(timestamp),
        Err(e) => ErrorCode::from(e).into(),
    }
}
