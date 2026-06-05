use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};

pub fn hash_password(password: &str) -> Result<String, anyhow::Error> {
    let mut salt_bytes = [0u8; 32];
    getrandom::getrandom(&mut salt_bytes).map_err(|e| {
        tracing::error!("failed to generate random salt: {}", e);
        anyhow::anyhow!("failed to generate random salt: {}", e)
    })?;
    let salt = SaltString::encode_b64(&salt_bytes).map_err(|e| {
        tracing::error!("salt encoding failed: {}", e);
        anyhow::anyhow!("salt encoding failed: {}", e)
    })?;
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| {
            tracing::error!("password hash failed: {}", e);
            anyhow::anyhow!("password hash failed: {}", e)
        })?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, anyhow::Error> {
    let parsed_hash = PasswordHash::new(hash).map_err(|e| {
        tracing::error!("invalid password hash: {}", e);
        anyhow::anyhow!("invalid password hash: {}", e)
    })?;
    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn generate_random_password() -> String {
    let mut bytes = [0u8; 16];
    if getrandom::getrandom(&mut bytes).is_ok() {
        hex::encode(bytes)
    } else {
        "admin123456".to_string()
    }
}
