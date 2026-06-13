use crate::{handlers, response::APIResponse, state::AppState};
use axum::{Router, middleware, routing};
use std::sync::Arc;

pub fn export(state: Arc<AppState>) -> Router {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .merge(public()) // Public routes
                .merge(protected(state.clone())) // Protected routes
                .merge(subscriber(state.clone())) // Subscriber Role routes
                .merge(
                    Router::new()
                        .route("/refresh", routing::get(handlers::refresh_token::refresh))
                        .layer(middleware::from_fn_with_state(
                            state.clone(),
                            crate::middleware::rate_limiter::export,
                        ))
                        .layer(middleware::from_fn_with_state(
                            state.clone(),
                            crate::middleware::refresh_auth::export,
                        )),
                ),
        )
        .with_state(state)
}

pub fn subscriber(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/market/finviz/screener",
            routing::post(handlers::market::finviz::screener),
        )
        .route(
            "/market/finviz/quote",
            routing::post(handlers::market::finviz::quote),
        )
        .route(
            "/market/finviz/news",
            routing::post(handlers::market::finviz::news),
        )
        .route(
            "/market/alpaca/snapshot",
            routing::post(handlers::market::alpaca::snapshot),
        )
        .route(
            "/tools/timestamp/akamai",
            routing::get(handlers::tools::timestamp::akamai),
        )
        .route(
            "/tools/calendar/tradingview_economic",
            routing::post(handlers::tools::calendar::tradingview_economic),
        )
        .layer(middleware::from_fn_with_state(
            state.clone(),
            crate::middleware::role::require_subscriber,
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            crate::middleware::rate_limiter::export,
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            crate::middleware::access_auth::export,
        ))
}

pub fn protected(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/users/me",
            routing::get(handlers::users::get_me).patch(handlers::users::update_me),
        )
        .route(
            "/users/change_password",
            routing::patch(handlers::users::change_password),
        )
        .route("/users/logout", routing::post(handlers::users::logout))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            crate::middleware::rate_limiter::export,
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            crate::middleware::access_auth::export,
        ))
}

pub fn public() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", routing::get(health))
        .route("/users/create", routing::post(handlers::users::create))
        .route("/users/login", routing::post(handlers::users::login))
}

pub async fn health() -> APIResponse<core_domain::result::HealthResult> {
    APIResponse::success(core_domain::result::HealthResult {
        status: true,
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
