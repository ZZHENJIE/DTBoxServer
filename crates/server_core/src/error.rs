use serde::Serialize;

use crate::response::APIResponse;

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorCode {
    DatabaseQuery,
    DatabaseWrite,
    UserNotExist,
    UserExist,
    TokenNotExist,
    NameNotMatch,
    PasswordNotMatch,
    Internal(String),
    TokenExpired,
    TokenRevoked,
    Unauthorized,
    RequestExtensions,
    TooManyRequests,
    JsonWebToken,
    Forbidden,
}

impl From<anyhow::Error> for ErrorCode {
    fn from(value: anyhow::Error) -> Self {
        ErrorCode::Internal(value.to_string())
    }
}

impl<T: Serialize> From<ErrorCode> for APIResponse<T> {
    fn from(value: ErrorCode) -> Self {
        match value {
            ErrorCode::DatabaseQuery => APIResponse::error(1, "Database query error"),
            ErrorCode::DatabaseWrite => APIResponse::error(2, "Database write error"),
            ErrorCode::UserNotExist => APIResponse::error(3, "User not exist"),
            ErrorCode::TokenNotExist => APIResponse::error(4, "Token not exist"),
            ErrorCode::NameNotMatch => APIResponse::error(5, "Name not match"),
            ErrorCode::PasswordNotMatch => APIResponse::error(6, "Password not match"),
            ErrorCode::UserExist => APIResponse::error(7, "User exist"),
            ErrorCode::Internal(value) => APIResponse::error(8, &value),
            ErrorCode::TokenExpired => APIResponse::error(10, "Token expired"),
            ErrorCode::TokenRevoked => APIResponse::error(11, "Token revoked"),
            ErrorCode::Unauthorized => APIResponse::error(12, "Unauthorized"),
            ErrorCode::RequestExtensions => APIResponse::error(13, "Request extensions error"),
            ErrorCode::TooManyRequests => APIResponse::error(14, "Too many requests"),
            ErrorCode::JsonWebToken => APIResponse::error(15, "Json web token error"),
            ErrorCode::Forbidden => APIResponse::error(16, "Forbidden error"),
        }
    }
}
