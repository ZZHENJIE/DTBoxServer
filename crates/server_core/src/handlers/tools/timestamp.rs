use std::sync::Arc;

use axum::extract::State;

use crate::{error::ErrorCode, response::APIResponse, state::AppState};

pub async fn akamai(State(state): State<Arc<AppState>>) -> APIResponse<u64> {
    match state.service().tools.timestamp.akamai().await {
        Ok(timestamp) => APIResponse::success(timestamp),
        Err(e) => ErrorCode::from(e).into(),
    }
}
