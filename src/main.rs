pub mod cli;
use std::sync::Arc;

use clap::Parser;
use server_core::{config, database, logging};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse command line arguments
    let cli = cli::Cli::parse();
    // Load config file
    let config = config::Config::load_file(&cli.config)?;
    info!("Load Config:{} success.", cli.config);
    // Setup logging
    let _guard = logging::setup(&config.logging);
    info!("Logging setup success.");
    // Connect to database
    let db = database::connect(&config.database_url).await?;
    info!("Database connection success.");
    // Initialize database tables
    let _ = database::init(&db).await?;
    info!("Database initialization success.");
    // Build routes
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let state = server_core::state::AppState::new(config, db);
    let routers = server_core::routers::export(Arc::new(state));
    // Start HTTP server
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Server starting on {}", addr);
    axum::serve(listener, routers).await?;
    Ok(())
}
