use sha2::Digest;
use uuid::Uuid;

pub fn generate_refresh_token() -> String {
    Uuid::new_v4().to_string()
}

pub fn refresh_token_hash(token: &str) -> String {
    hex::encode(sha2::Sha256::digest(token.as_bytes()))
}
