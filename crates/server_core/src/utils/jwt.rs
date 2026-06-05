use crate::config::JWTConfig;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32, // user_id
    pub role: u8,
    pub exp: usize, // expiry timestamp (seconds)
    pub iat: usize, // issued at timestamp (seconds)
}

/// Create a signed JWT access token.
pub fn create_access_token(
    user_id: i32,
    role: u8,
    config: &JWTConfig,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = chrono::Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: user_id,
        role,
        exp: now + config.access_token_expire_minutes as usize * 60,
        iat: now,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
}

/// Verify and decode a JWT access token.
pub fn verify_access_token(
    token: &str,
    secret: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::default();
    validation.validate_exp = true;
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
}
