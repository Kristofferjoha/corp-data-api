// "$env:RUST_LOG="info,corp_data_api=debug,sqlx=info";" for logging
use dotenv::dotenv;
use std::sync::Arc;
use tokio::net::TcpListener;

mod config;
mod entity;
mod repository;
mod service;
mod controller;
mod dto;
mod utils;

use config::db_settings::Settings;
use repository::office_repository::OfficeRepository;
use repository::employee_repository::EmployeeRepository;
use service::office_service::OfficeService;
use service::employee_service::EmployeeService;
use controller::office_controller::create_router as create_office_router;
use controller::employee_controller::create_router as create_employee_router;

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
    let office_repo = OfficeRepository::new(pool.clone());
    let employee_repo = EmployeeRepository::new(pool.clone());
    let office_service = Arc::new(OfficeService::new(office_repo.clone()));
    let employee_service = Arc::new(EmployeeService::new(employee_repo, office_repo));

    // builds HTTP layer
    let app = create_office_router(office_service).merge(create_employee_router(employee_service));

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