use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{error::ErrorCode, response::APIResponse, state::AppState};

pub async fn require_admin(
    State(_): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, Response> {
    if let Some(auth) = req.extensions().get::<crate::AuthContext>() {
        if auth.role < 5 {
            return Err(APIResponse::<()>::from(ErrorCode::Forbidden).into_response());
        }
    } else {
        return Err(APIResponse::<()>::from(ErrorCode::Unauthorized).into_response());
    }
    Ok(next.run(req).await)
}

pub async fn require_subscriber(
    State(_): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, Response> {
    if let Some(auth) = req.extensions().get::<crate::AuthContext>() {
        if auth.role < 2 {
            return Err(APIResponse::<()>::from(ErrorCode::Forbidden).into_response());
        }
    } else {
        return Err(APIResponse::<()>::from(ErrorCode::Unauthorized).into_response());
    }
    Ok(next.run(req).await)
}
