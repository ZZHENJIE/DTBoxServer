pub mod entity {
    pub mod refresh_token;
    pub mod stocks;
    pub mod users;
}

pub mod payload {
    pub mod stocks;
    pub mod users;
}

pub mod query {
    pub mod stocks;
    pub mod users;
}

pub mod result {
    pub mod stocks;
    pub mod users;
}

#[derive(Debug, serde::Serialize)]
pub struct HealthResult {
    pub status: bool,
    pub version: String,
}
