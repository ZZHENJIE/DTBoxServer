use crate::{error::ErrorCode, response::APIResponse, state::AppState};
use axum::{Json, extract::State};
use std::sync::Arc;

pub async fn snapshot(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<alpaca_sdk::SnapshotQuery>,
) -> APIResponse<alpaca_sdk::snapshot::Response> {
    match state.service().market.alpaca_sdk.snapshot(&payload).await {
        Ok(result) => APIResponse::success(result),
        Err(e) => ErrorCode::from(e).into(),
    }
}
