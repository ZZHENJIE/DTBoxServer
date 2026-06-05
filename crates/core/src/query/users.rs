use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NameExistsQuery {
    pub name: String,
}
