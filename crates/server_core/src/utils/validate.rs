pub fn validate_name(name: &str) -> bool {
    let len = name.len();
    if len < 5 || len > 15 {
        return false;
    }
    name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn validate_password(password: &str) -> bool {
    let len = password.chars().count();
    if len < 6 || len > 32 {
        return false;
    }
    password
        .chars()
        .all(|c| c.is_ascii_graphic() || c.is_ascii_alphanumeric())
}
