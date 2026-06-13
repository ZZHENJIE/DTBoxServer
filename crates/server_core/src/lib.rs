pub mod config;
pub mod database;
pub mod error;
pub mod logging;
pub mod response;
pub mod routers;
pub mod state;
//
pub mod utils {
    pub mod auth;
    pub mod jwt;
    pub mod rate_limiter;
    pub mod token;
    pub mod validate;
}
//
pub mod middleware {
    pub mod access_auth;
    pub mod rate_limiter;
    pub mod refresh_auth;
    pub mod role;
}
//
pub mod service {
    pub mod market;
    pub mod refresh_token;
    pub mod stocks;
    pub mod tools;
    pub mod users;
}
//
pub mod handlers {
    pub mod admin;
    pub mod refresh_token;
    pub mod stocks;
    pub mod users;
    pub mod market {
        pub mod alpaca;
        pub mod finviz;
    }
    pub mod tools {
        pub mod calendar;
        pub mod timestamp;
    }
}

pub use state::AuthContext;
