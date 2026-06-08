use std::sync::Arc;

use axum::{Extension, Json, extract::State, http::header, response::IntoResponse};
use core_db::entity::{refresh_token, users};
use core_domain::{payload, result};

use crate::{error::ErrorCode, response::APIResponse, state::AppState, utils};

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<payload::users::CreatePayload>,
) -> APIResponse<result::users::InfoResult> {
    // Validate name format
    if !utils::validate::validate_name(&payload.name) {
        return ErrorCode::NameNotMatch.into();
    }

    // Validate password format
    if !utils::validate::validate_password(&payload.password) {
        return ErrorCode::PasswordNotMatch.into();
    }

    // Check if user already exists
    match state.service().users.find_with_name(&payload.name).await {
        Ok(_) => ErrorCode::UserExist.into(),
        Err(ErrorCode::UserNotExist) => {
            // Hash password before storing
            let password_hash = match utils::auth::hash_password(&payload.password) {
                Ok(hash) => hash,
                Err(err) => return ErrorCode::from(err).into(),
            };

            // Build user model
            let model = users::ActiveModel {
                name: sea_orm::Set(payload.name),
                password_hash: sea_orm::Set(password_hash),
                avatar: sea_orm::Set(String::new()),
                role: sea_orm::Set(users::Role::User),
                settings: sea_orm::Set(core_domain::UserSettings::default().value()),
                created_at: sea_orm::Set(chrono::Utc::now().naive_utc()),
                ..Default::default()
            };

            // Insert into database
            let info = match state.service().users.create(model).await {
                Ok(user) => user,
                Err(err) => return err.into(),
            };

            APIResponse::success(info.into())
        }
        Err(err) => err.into(),
    }
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<payload::users::LoginPayload>,
) -> axum::response::Response {
    // Find user by name
    let user = match state.service().users.find_with_name(&payload.name).await {
        Ok(user) => user,
        Err(err) => return APIResponse::<bool>::from(err).into_response(),
    };

    // Verify password
    let is_password_match =
        match utils::auth::verify_password(&payload.password, &user.password_hash) {
            Ok(value) => value,
            Err(err) => return APIResponse::<bool>::from(ErrorCode::from(err)).into_response(),
        };

    if !is_password_match {
        return APIResponse::<bool>::from(ErrorCode::PasswordNotMatch).into_response();
    }

    // Generate refresh token
    let mut is_first_login = false;
    let refresh_token_expire_days = state.config().jwt.refresh_token_expire_days as i64;
    let token = utils::token::generate_refresh_token();
    let token_hash = utils::token::refresh_token_hash(&token);
    let expires_at =
        chrono::Utc::now().naive_utc() + chrono::Duration::days(refresh_token_expire_days);

    // Store refresh token (create if first login, otherwise update)
    match state
        .service()
        .refresh_token
        .find_with_user_id(user.id)
        .await
    {
        Ok(_) => {}
        Err(ErrorCode::TokenNotExist) => {
            let _ = match state
                .service()
                .refresh_token
                .create(refresh_token::ActiveModel {
                    user_id: sea_orm::Set(user.id),
                    token_hash: sea_orm::Set(token_hash.clone()),
                    revoked: sea_orm::Set(false),
                    created_at: sea_orm::Set(chrono::Utc::now().naive_utc()),
                    expires_at: sea_orm::Set(expires_at),
                    ..Default::default()
                })
                .await
            {
                Ok(_) => is_first_login = true,
                Err(err) => return APIResponse::<bool>::from(err).into_response(),
            };
        }
        Err(err) => return APIResponse::<bool>::from(err).into_response(),
    };

    if !is_first_login {
        let _ = match state
            .service()
            .refresh_token
            .update(user.id, token_hash, expires_at)
            .await
        {
            Ok(_) => {}
            Err(err) => return APIResponse::<bool>::from(err).into_response(),
        };
    }

    // Set refresh_token cookie
    let mut response = APIResponse::<bool>::success(true).into_response();
    let max_age = refresh_token_expire_days * 86400;
    let cookie = format!(
        "refresh_token={}; HttpOnly; Path=/; Max-Age={}; SameSite=Strict",
        token, max_age,
    );

    if let Ok(val) = cookie.parse() {
        response.headers_mut().insert(header::SET_COOKIE, val);
    }

    response
}

pub async fn get_me(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
) -> APIResponse<result::users::InfoResult> {
    match state.service().users.find_with_id(user_id).await {
        Ok(value) => APIResponse::success(value.into()),
        Err(err) => APIResponse::from(err),
    }
}

pub async fn update_me(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
    Json(payload): Json<payload::users::UpdateUserPayload>,
) -> APIResponse<result::users::InfoResult> {
    match state.service().users.update(user_id, payload).await {
        Ok(value) => APIResponse::success(value.into()),
        Err(err) => APIResponse::from(err),
    }
}

pub async fn logout(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
) -> APIResponse<bool> {
    match state.service().refresh_token.revoke(user_id).await {
        Ok(_) => APIResponse::success(true),
        Err(err) => APIResponse::from(err),
    }
}

pub async fn change_password(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
    Json(payload): Json<payload::users::ChangePasswordPayload>,
) -> APIResponse<bool> {
    // 检查新密码是否与旧密码相同
    if payload.new_password == payload.old_password {
        return APIResponse::from(ErrorCode::PasswordNotMatch);
    }
    // 获取用户实体
    let user = match state.service().users.find_with_id(user_id).await {
        Ok(value) => value,
        Err(err) => return APIResponse::from(err),
    };
    // 判断旧密码是否正确
    let is_password_match =
        match utils::auth::verify_password(&payload.old_password, &user.password_hash) {
            Ok(value) => value,
            Err(err) => return APIResponse::from(ErrorCode::from(err)),
        };

    if !is_password_match {
        return APIResponse::from(ErrorCode::PasswordNotMatch);
    }
    // 撤销现在的Refresh Token并把新密码写入数据库
    let new_password_hash = match utils::auth::hash_password(&payload.new_password) {
        Ok(hash) => hash,
        Err(err) => return ErrorCode::from(err).into(),
    };

    let _ = match state.service().refresh_token.revoke(user_id).await {
        Ok(_) => {}
        Err(err) => return APIResponse::from(err),
    };

    match state
        .service()
        .users
        .change_password(user_id, new_password_hash)
        .await
    {
        Ok(_) => APIResponse::success(true),
        Err(err) => APIResponse::from(err),
    }
}
