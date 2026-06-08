use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreatePayload {
    pub name: String,
    pub password: String,
    pub settings: crate::UserSettings,
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum UpdateUserPayload {
    Name(String),
    Settings(crate::UserSettings),
    Avatar(String),
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordPayload {
    pub old_password: String,
    pub new_password: String,
}
