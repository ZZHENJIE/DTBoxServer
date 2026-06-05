use crate::{
    config::{self, Config},
    service,
    utils::rate_limiter::RateLimiter,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub rate_limiter: RateLimiter,
    config: Arc<Config>,
    service: Arc<Service>,
}

#[derive(Clone)]
pub struct Service {
    pub database: sea_orm::DatabaseConnection,
    pub users: service::users::UsersService,
    pub refresh_token: service::refresh_token::RefreshTokenService,
    pub stocks: service::stocks::StocksService,
    pub market: service::market::MarketService,
    pub tools: service::tools::ToolsService,
}

impl AppState {
    pub fn new(config: config::Config, database: sea_orm::DatabaseConnection) -> Self {
        let service = Service {
            users: service::users::UsersService::new(database.clone()),
            refresh_token: service::refresh_token::RefreshTokenService::new(database.clone()),
            stocks: service::stocks::StocksService::new(database.clone()),
            market: service::market::MarketService::new(&config.data_source),
            tools: service::tools::ToolsService::new(),
            database,
        };
        let rate_limiter = RateLimiter::new(
            config.rate_limiter.max_requests,
            config.rate_limiter.window_seconds,
        );

        Self {
            config: Arc::new(config),
            service: Arc::new(service),
            rate_limiter,
        }
    }
    pub fn config(&self) -> Arc<Config> {
        self.config.clone()
    }
    pub fn service(&self) -> Arc<Service> {
        self.service.clone()
    }
}
