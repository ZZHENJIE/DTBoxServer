use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UserSettings {}

impl Default for UserSettings {
    fn default() -> Self {
        Self {}
    }
}

impl UserSettings {
    pub fn value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}
