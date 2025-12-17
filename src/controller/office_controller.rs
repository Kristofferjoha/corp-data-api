use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
    response::IntoResponse,
    http::StatusCode,
};
use std::sync::Arc;
use crate::service::office_service::OfficeService;
use crate::entity::office::Office;
use crate::dto::office_dto::CreateOfficeRequest;

pub fn create_router(service: Arc<OfficeService>) -> Router {
    Router::new()
        .route("/offices", post(create_office).get(list_all_offices))
        .route("/offices/{id}", get(get_office_by_id))
        .with_state(service)
}

async fn create_office(
    State(service): State<Arc<OfficeService>>,
    Json(req): Json<CreateOfficeRequest>,
) -> impl IntoResponse {
    tracing::info!("Received request to create office: {}", req.name);
    let office = Office::from_create_request(req);
    
    match service.add_office(&office).await {
        Ok(new_office) => {
            tracing::info!("Successfully created office with ID: {:?}", new_office.id.unwrap());
            (StatusCode::CREATED, Json(new_office.to_response())).into_response()
        },
        Err(e) => {
            tracing::warn!("Failed to process office creation: {}", e);
            (StatusCode::BAD_REQUEST, Json(e.to_string())).into_response()
        }
    }
}

async fn get_office_by_id(
    State(service): State<Arc<OfficeService>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match service.find_office_by_id(id).await {
        Ok(Some(office)) => {
            tracing::info!("Office with id {} found", id);
            Json(office.to_response()).into_response()
        }
        Ok(None) => {
            tracing::warn!("Office with id {} not found", id);
            (StatusCode::NOT_FOUND, "Office not found").into_response()
        }
        Err(e) => {
            tracing::error!("Error finding office {}: {}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

async fn list_all_offices(
    State(service): State<Arc<OfficeService>>,
) -> impl IntoResponse {
    match service.list_all_offices().await {
        Ok(offices) => {
            let responses: Vec<_> = offices.into_iter().map(|o| o.to_response()).collect();
            tracing::info!("Found a total of {} offices", responses.len());
            Json(responses).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list offices: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        },
    }
}