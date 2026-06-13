use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{error::ErrorCode, response::APIResponse, state::AppState, utils};

pub async fn export(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, Response> {
    let raw_token = req
        .headers()
        .get("cookie")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| {
            s.split(';')
                .filter_map(|p| p.split_once('='))
                .find(|(k, _)| k.trim().eq_ignore_ascii_case("refresh_token"))
                .map(|(_, v)| v.trim().to_string())
        })
        .ok_or_else(|| APIResponse::<()>::from(ErrorCode::Unauthorized).into_response())?;

    let token_hash = utils::token::refresh_token_hash(&raw_token);

    let token = state
        .service()
        .refresh_token
        .find_with_token_hash(&token_hash)
        .await
        .map_err(|err| APIResponse::<()>::from(err).into_response())?;

    if token.revoked || token.expires_at < chrono::Utc::now().naive_utc() {
        return Err(APIResponse::<()>::from(ErrorCode::Unauthorized).into_response());
    }

    match state.service().users.find_with_id(token.user_id).await {
        Ok(user) => {
            req.extensions_mut().insert(crate::AuthContext {
                user_id: user.id,
                role: user.role.into(),
            });
            Ok(next.run(req).await)
        }
        Err(err) => Err(APIResponse::<()>::from(err).into_response()),
    }
}
