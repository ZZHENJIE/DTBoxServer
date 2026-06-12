use std::sync::Arc;

use axum::{Json, extract::State};
use core_domain::{payload, result};

use crate::{error::ErrorCode, response::APIResponse, state::AppState};

pub async fn tradingview_economic(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<payload::tools::TradingviewEconomicCalendarPayload>,
) -> APIResponse<Vec<result::tools::TradingviewEconomicCalendarItem>> {
    match state
        .service()
        .tools
        .calendar
        .tradingview_economic(payload.from, payload.to)
        .await
    {
        Ok(value) => APIResponse::success(value),
        Err(e) => ErrorCode::from(e).into(),
    }
}
