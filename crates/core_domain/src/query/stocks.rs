use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub key_word: String,
    pub limit: usize,
}
