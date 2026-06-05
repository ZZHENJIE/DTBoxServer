use sea_orm::JsonValue;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct InfoResult {
    pub id: i32,
    pub name: String,
    pub avatar: String,
    pub role: crate::entity::users::Role,
    pub settings: JsonValue,
    pub created_at: chrono::NaiveDateTime,
}

impl From<crate::entity::users::Model> for InfoResult {
    fn from(value: crate::entity::users::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            avatar: value.avatar,
            role: value.role,
            settings: value.settings,
            created_at: value.created_at,
        }
    }
}
