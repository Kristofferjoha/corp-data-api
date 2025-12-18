use std::sync::Arc;
use axum::{body::Body, http::{Request, StatusCode}, Router};
use tower::util::ServiceExt;
use serial_test::serial;
use serde_json::json;

use corp_data_api::config::db_settings::Settings;
use corp_data_api::repository::office_repository::OfficeRepository;
use corp_data_api::service::office_service::OfficeService;
use corp_data_api::controller::office_controller::create_router;

mod utils;
use utils::clean_db;

/// Tests for office endpoints
/// Should cover everything if production code ofc


/// Test http POST /Offices now that endpoint exists 
/// Expects 201 Created on success
#[tokio::test]
#[serial]
async fn test_create_office_endpoint_test() {
    dotenv::from_filename(".env.test").ok();
    let pool = Settings::connect_from_env().unwrap().create_pool().await.unwrap();
    clean_db(&pool).await;

    let repo = OfficeRepository::new(pool.clone());
    let service = Arc::new(OfficeService::new(repo.clone()));
    let app: Router = create_router(service);

    let office_payload = json!({
        "name": "Vester Hassing",
        "max_occupancy": 42
    });

    let request = Request::builder()
        .method("POST")
        .uri("/offices")
        .header("content-type", "application/json")
        .body(Body::from(office_payload.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    clean_db(&pool).await;
}

/// Test http DELETE /offices/{id} for non existent office
/// Expects 404 Not Found
#[tokio::test]
#[serial]
async fn delete_office_not_found_endpoint_test() {
    dotenv::from_filename(".env.test").ok();
    let pool = Settings::connect_from_env().unwrap().create_pool().await.unwrap();
    clean_db(&pool).await;

    let repo = OfficeRepository::new(pool.clone());
    let service = Arc::new(OfficeService::new(repo.clone()));
    let app: Router = create_router(service);

    let request = Request::builder()
        .method("DELETE")
        .uri("/offices/33")
        .body(axum::body::Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    clean_db(&pool).await;
}
