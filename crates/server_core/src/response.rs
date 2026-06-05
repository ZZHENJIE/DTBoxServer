use axum::{Json, response::IntoResponse};
use reqwest::StatusCode;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct APIResponse<T: Serialize> {
    pub code: i32,
    pub data: Option<T>,
    pub message: String,
}

impl<T: Serialize> APIResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            data: Some(data),
            message: "success".to_string(),
        }
    }

    pub fn error(code: i32, message: &str) -> Self {
        Self {
            code,
            data: None,
            message: message.to_string(),
        }
    }
}

impl<T: Serialize> IntoResponse for APIResponse<T> {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
