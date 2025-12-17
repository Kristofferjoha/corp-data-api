//! Entry point for the Corp Data API application.
//! "$env:RUST_LOG="info,corp_data_api=debug,sqlx=info";" for logging
use dotenv::dotenv;
use std::sync::Arc;
use tokio::net::TcpListener;

mod config;
mod entity;
mod repository;
mod service;
mod controller;
mod dto;

use config::db_settings::Settings;
use repository::office_repository::OfficeRepository;
use service::office_service::OfficeService;
use controller::office_controller::create_router;

/// Initializes and runs the Corp Data API server.
/// Initializes structured logging, loads environment configuration,
/// establishes a database connection pool, configures the application
/// router, and starts listening for incoming HTTP requests.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // load enviornment variables from .env if possible
    dotenv().ok();

    // Initialize structured logging from the `RUST_LOG` environment variable
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Load application configuration from environment
    let settings = Settings::connect_from_env().map_err(|e| {
        tracing::error!("Failed to load settings: {}", e);
        e
    })?;

    // Create a Postgres connection pool
    let pool = settings.create_pool().await.map_err(|e| {
        tracing::error!("Database connection failed: {}", e);
        e
    })?;

    // Initialize repository and service layers
    let repo = OfficeRepository::new(pool);
    let service = Arc::new(OfficeService::new(repo));

    // builds HTTP layer
    let app = create_router(service);

    // TCP listener binding to address and HTTP server startup
    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr).await.map_err(|e| {
        tracing::error!("Failed to bind to {}: {}", addr, e);
        e
    })?;
    tracing::info!("listening on http://{}", addr);
    
    // Start the Axum server
    axum::serve(listener, app).await?;

    Ok(())
}