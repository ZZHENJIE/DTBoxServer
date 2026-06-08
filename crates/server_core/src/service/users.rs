use core_db::entity::users;
use core_domain::payload::users::UpdateUserPayload;

use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};

use crate::error::ErrorCode;

#[derive(Clone)]
pub struct UsersService {
    pub database: sea_orm::DatabaseConnection,
}

impl UsersService {
    pub fn new(database: sea_orm::DatabaseConnection) -> Self {
        Self { database }
    }
    pub async fn find_with_name(&self, name: &str) -> Result<users::Model, ErrorCode> {
        users::Entity::find()
            .filter(users::Column::Name.eq(name))
            .one(&self.database)
            .await
            .map_err(|err| {
                tracing::error!("Database query error: {}", err);
                ErrorCode::DatabaseQuery
            })?
            .ok_or(ErrorCode::UserNotExist)
    }
    pub async fn find_with_id(&self, id: i32) -> Result<users::Model, ErrorCode> {
        users::Entity::find()
            .filter(users::Column::Id.eq(id))
            .one(&self.database)
            .await
            .map_err(|err| {
                tracing::error!("Database query error: {}", err);
                ErrorCode::DatabaseQuery
            })?
            .ok_or(ErrorCode::UserNotExist)
    }
    pub async fn create(&self, user: users::ActiveModel) -> Result<users::Model, ErrorCode> {
        user.insert(&self.database).await.map_err(|err| {
            tracing::error!("Database write error: {}", err);
            ErrorCode::DatabaseWrite
        })
    }
    pub async fn update(
        &self,
        id: i32,
        update_data: UpdateUserPayload,
    ) -> Result<users::Model, ErrorCode> {
        let mut user: users::ActiveModel = self.find_with_id(id).await?.into();
        match update_data {
            UpdateUserPayload::Name(name) => {
                user.name = sea_orm::Set(name);
            }
            UpdateUserPayload::Settings(settings) => {
                user.settings = sea_orm::Set(settings.value());
            }
            UpdateUserPayload::Avatar(avatar) => {
                user.avatar = sea_orm::Set(avatar);
            }
        }
        user.update(&self.database).await.map_err(|err| {
            tracing::error!("Database write error: {}", err);
            ErrorCode::DatabaseWrite
        })
    }
    pub async fn change_password(
        &self,
        id: i32,
        password_hash: String,
    ) -> Result<users::Model, ErrorCode> {
        let mut user: users::ActiveModel = self.find_with_id(id).await?.into();
        user.password_hash = sea_orm::Set(password_hash);
        user.update(&self.database).await.map_err(|err| {
            tracing::error!("Database write error: {}", err);
            ErrorCode::DatabaseWrite
        })
    }
    pub async fn paginate(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<users::Model>, ErrorCode> {
        users::Entity::find()
            .paginate(&self.database, page_size)
            .fetch_page(page - 1)
            .await
            .map_err(|err| {
                tracing::error!("Database query error: {}", err);
                ErrorCode::DatabaseQuery
            })
    }
}
