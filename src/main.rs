// "$env:RUST_LOG="info,corp_data_api=debug,sqlx=info";" for logging
use dotenv::dotenv;
use std::sync::Arc;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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
use controller::employee_controller::{create_router as create_employee_router};
use dto::employee_dto::{EmployeeResponse, CreateEmployeeRequest};
use dto::office_dto::{OfficeResponse, CreateOfficeRequest};


/// OA specs for api
#[derive(OpenApi)]
#[openapi(
    paths(
        controller::employee_controller::get_employee_by_id,
        controller::employee_controller::create_employee,
        controller::employee_controller::list_all_employees,
        controller::employee_controller::list_employees_by_office_id,
        controller::employee_controller::update_employee,
        controller::employee_controller::delete_employee,
        controller::office_controller::create_office,
        controller::office_controller::get_office_by_id,
        controller::office_controller::list_all_offices,
        controller::office_controller::update_office,
        controller::office_controller::delete_office
    ),
    components(schemas(EmployeeResponse, CreateEmployeeRequest, OfficeResponse, CreateOfficeRequest))
)]
struct ApiDoc;

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
    let app = create_office_router(office_service)
        .merge(create_employee_router(employee_service))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

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