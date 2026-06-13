use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{error::ErrorCode, response::APIResponse, state::AppState};

pub async fn export(
    State(state): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, Response> {
    if let Some(auth) = req.extensions().get::<crate::AuthContext>() {
        if !state.rate_limiter.check(auth.user_id).await {
            return Err(APIResponse::<()>::from(ErrorCode::TooManyRequests).into_response());
        }
        Ok(next.run(req).await)
    } else {
        Err(APIResponse::<()>::from(ErrorCode::RequestExtensions).into_response())
    }
}
