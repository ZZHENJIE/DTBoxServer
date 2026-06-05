use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::errors::ErrorKind;

use crate::{error::ErrorCode, response::APIResponse, state::AppState, utils};

pub async fn export(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, Response> {
    let token = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| APIResponse::<()>::from(ErrorCode::Unauthorized).into_response())?;

    let claims = match utils::jwt::verify_access_token(token, &state.config().jwt.secret) {
        Ok(claims) => claims,
        Err(err) => match err.kind() {
            ErrorKind::ExpiredSignature => {
                return Err(APIResponse::<()>::from(ErrorCode::TokenExpired).into_response());
            }
            _ => {
                return Err(APIResponse::<()>::from(ErrorCode::Unauthorized).into_response());
            }
        },
    };

    req.extensions_mut().insert(claims.sub);

    Ok(next.run(req).await)
}
