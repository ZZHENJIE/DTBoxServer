use std::sync::Arc;

use axum::{Extension, extract::State};

use crate::{error::ErrorCode, response::APIResponse, state::AppState, utils};

pub async fn refresh(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<crate::AuthContext>,
) -> APIResponse<String> {
    let user = match state.service().users.find_with_id(auth.user_id).await {
        Ok(value) => value,
        Err(err) => return APIResponse::from(err),
    };

    let token =
        match utils::jwt::create_access_token(user.id, user.role.into(), &state.config().jwt) {
            Ok(value) => value,
            Err(err) => {
                tracing::error!("json web token:{}", err.to_string());
                return APIResponse::from(ErrorCode::JsonWebToken);
            }
        };

    APIResponse::success(token)
}
