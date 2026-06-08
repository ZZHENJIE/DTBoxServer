pub mod payload {
    pub mod stocks;
    pub mod tools;
    pub mod users;
}

pub mod query {
    pub mod stocks;
    pub mod users;
}

pub mod user_settings;

pub mod result {
    pub mod stocks;
    pub mod users;

    #[derive(Debug, serde::Serialize)]
    pub struct HealthResult {
        pub status: bool,
        pub version: String,
    }
}

pub use user_settings::UserSettings;
