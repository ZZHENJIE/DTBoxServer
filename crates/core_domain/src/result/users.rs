use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct InfoResult {
    pub id: i32,
    pub name: String,
    pub avatar: String,
    pub role: u8,
    pub settings: crate::UserSettings,
    pub created_at: chrono::NaiveDateTime,
}
