use crate::{error::ErrorCode, response::APIResponse, state::AppState};
use axum::{Json, extract::State};
use std::sync::Arc;

pub async fn screener(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<finviz_sdk::ScreenerQuery>,
) -> APIResponse<finviz_sdk::screener::JSONResult> {
    match state.service().market.finviz.screener(&payload).await {
        Ok(result) => APIResponse::success(result),
        Err(e) => ErrorCode::from(e).into(),
    }
}

pub async fn quote(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<finviz_sdk::QuoteQuery>,
) -> APIResponse<finviz_sdk::quote::JSONResult> {
    match state.service().market.finviz.quote(&payload).await {
        Ok(result) => APIResponse::success(result),
        Err(e) => ErrorCode::from(e).into(),
    }
}

pub async fn news(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<finviz_sdk::NewsQuery>,
) -> APIResponse<finviz_sdk::news::JSONResult> {
    match state.service().market.finviz.news(&payload).await {
        Ok(result) => APIResponse::success(result),
        Err(e) => ErrorCode::from(e).into(),
    }
}
