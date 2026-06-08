use core_db::entity::refresh_token;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

use crate::error::ErrorCode;

#[derive(Clone)]
pub struct RefreshTokenService {
    pub database: sea_orm::DatabaseConnection,
}

impl RefreshTokenService {
    pub fn new(database: sea_orm::DatabaseConnection) -> Self {
        Self { database }
    }
    pub async fn find_with_user_id(&self, user_id: i32) -> Result<refresh_token::Model, ErrorCode> {
        refresh_token::Entity::find()
            .filter(refresh_token::Column::UserId.eq(user_id))
            .one(&self.database)
            .await
            .map_err(|err| {
                tracing::error!("Database query error: {}", err);
                ErrorCode::DatabaseQuery
            })?
            .ok_or(ErrorCode::TokenNotExist)
    }
    pub async fn find_with_token_hash(
        &self,
        token_hash: &str,
    ) -> Result<refresh_token::Model, ErrorCode> {
        refresh_token::Entity::find()
            .filter(refresh_token::Column::TokenHash.eq(token_hash))
            .one(&self.database)
            .await
            .map_err(|err| {
                tracing::error!("Database query error: {}", err);
                ErrorCode::DatabaseQuery
            })?
            .ok_or(ErrorCode::TokenNotExist)
    }
    pub async fn create(
        &self,
        model: refresh_token::ActiveModel,
    ) -> Result<refresh_token::Model, ErrorCode> {
        model.insert(&self.database).await.map_err(|err| {
            tracing::error!("Database write error: {}", err);
            ErrorCode::DatabaseWrite
        })
    }
    pub async fn revoke(&self, user_id: i32) -> Result<refresh_token::Model, ErrorCode> {
        let mut model: refresh_token::ActiveModel = self.find_with_user_id(user_id).await?.into();
        model.revoked = sea_orm::Set(true);
        model.update(&self.database).await.map_err(|err| {
            tracing::error!("Database write error: {}", err);
            ErrorCode::DatabaseWrite
        })
    }
    pub async fn update(
        &self,
        user_id: i32,
        token_hash: String,
        expires_at: chrono::NaiveDateTime,
    ) -> Result<refresh_token::Model, ErrorCode> {
        let mut model: refresh_token::ActiveModel = self.find_with_user_id(user_id).await?.into();
        model.token_hash = sea_orm::Set(token_hash);
        model.revoked = sea_orm::Set(false);
        model.expires_at = sea_orm::Set(expires_at);
        model.created_at = sea_orm::Set(chrono::Utc::now().naive_utc());
        model.update(&self.database).await.map_err(|err| {
            tracing::error!("Database write error: {}", err);
            ErrorCode::DatabaseWrite
        })
    }
}
